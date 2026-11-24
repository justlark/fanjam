// The refresh coordinator: a Durable Object, one per environment, that owns every fetch of data
// from NocoDB.
//
// The edge cache is scoped to a datacenter, and the bookkeeping in `store::REFRESH_STATE` is scoped
// to a single isolate. Under heavy load, Cloudflare runs our worker in a great many isolates, and
// if each of them fetched from NocoDB on its own, NocoDB would see one burst of requests per
// isolate per TTL. That's enough to knock it over.
//
// So instead, isolates ask the coordinator for the latest data, and the coordinator fetches from
// NocoDB on their behalf: at most once per TTL, with every concurrent caller sharing the one fetch,
// and with a single backoff that covers everyone. It's also the only thing that writes the
// persistent cache in KV. Isolates still populate their own datacenter's edge cache from what the
// coordinator hands back.
//
// Everything here lives in memory. KV is still the persistent copy, so if Cloudflare evicts the
// coordinator, the next request just fetches again.

use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::rc::Rc;
use std::time::Duration;

use anyhow::{Context as _, anyhow};
use futures::future::{FutureExt, LocalBoxFuture, Shared};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use worker::kv::KvStore;
use worker::wasm_bindgen::JsValue;
use worker::{
    DurableObject, Env, Method, ObjectNamespace, Request, RequestInit, Response, State,
    durable_object,
};

use crate::env::EnvName;
use crate::log;
use crate::noco::{self, BaseId, Client as NocoClient};
use crate::store::{Store, TimedOut, backoff_delay, now_ms, with_timeout};
use crate::{config, kv, url};

// The time (as epoch milliseconds) the data in a response was fetched from NocoDB.
const FETCHED_AT_HEADER: &str = "X-Fetched-At";

// The hash of the snapshot in a response, which the caller can send back as `known_hash` next
// time. See `FetchRequest::known_hash`.
const SNAPSHOT_HASH_HEADER: &str = "X-Snapshot-Hash";

// The data endpoints the coordinator fetches.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Endpoint {
    Events,
    People,
    Locations,
    Info,
    Pages,
    Announcements,
    Files,
    Vendors,
    VendorPages,
    Passwords,
}

impl Endpoint {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Events => "events",
            Self::People => "people",
            Self::Locations => "locations",
            Self::Info => "info",
            Self::Pages => "pages",
            Self::Announcements => "announcements",
            Self::Files => "files",
            Self::Vendors => "vendors",
            Self::VendorPages => "vendor_pages",
            Self::Passwords => "passwords",
        }
    }

    fn cache_key(self, env_name: &EnvName) -> String {
        match self {
            Self::Events => kv::events_cache_key(env_name),
            Self::People => kv::people_cache_key(env_name),
            Self::Locations => kv::locations_cache_key(env_name),
            Self::Info => kv::info_cache_key(env_name),
            Self::Pages => kv::pages_cache_key(env_name),
            Self::Announcements => kv::announcements_cache_key(env_name),
            Self::Files => kv::files_cache_key(env_name),
            Self::Vendors => kv::vendors_cache_key(env_name),
            Self::VendorPages => kv::vendor_pages_cache_key(env_name),
            Self::Passwords => kv::passwords_cache_key(env_name),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct FetchRequest {
    env_name: EnvName,
    endpoint: Endpoint,

    // The caller's `Store` has this on hand, and seeds it from Postgres when it's missing from KV.
    // Clearing the cache deletes it from KV, so the coordinator can't count on finding it there.
    base_id: String,

    // How old the data may be and still be handed back without fetching again. This is the
    // environment's edge cache TTL, which the caller already has on hand.
    ttl_ms: i64,

    // Fetch from NocoDB even if the data we have is younger than the TTL. Used when we know it's
    // out of date, like when a new announcement has just been posted.
    force: bool,

    // The hash of the snapshot the caller already has, if any. When the latest snapshot has the
    // same hash, we answer with a 304 and its fetch time instead of the whole body.
    //
    // Every refresh-ahead from every isolate comes through here, and this object handles one
    // request at a time. Sending the full body back on each of those calls, even though the data
    // rarely changes, is what makes this object the bottleneck under heavy load.
    //
    // Defaulted so that a request from an older deploy of the worker, which doesn't send it, still
    // parses.
    #[serde(default)]
    known_hash: Option<String>,
}

// What the coordinator hands back to an isolate.
pub enum Fetched<T> {
    // The latest data.
    Value {
        value: T,

        // When the coordinator started the fetch that produced this value, as epoch
        // milliseconds.
        fetched_at_ms: i64,

        // The hash of the snapshot this came from, to pass back as `known_hash` next time.
        hash: Option<String>,
    },

    // The latest data is the same as what the caller said it has (see `known_hash`), so here's
    // just its fetch time.
    NotModified {
        fetched_at_ms: i64,
    },
}

// Get the latest data for an endpoint by way of the coordinator, which only goes to NocoDB if
// what it has is older than `ttl` (or `force` is set). Fails if the coordinator's fetch failed or
// it's in backoff after repeated failures.
//
// Pass `known_hash` (from an earlier `Fetched::Value`) to get `Fetched::NotModified` back instead
// of the data, if it hasn't changed.
pub async fn fetch<T: DeserializeOwned>(
    namespace: &ObjectNamespace,
    env_name: &EnvName,
    base_id: &BaseId,
    endpoint: Endpoint,
    ttl: Duration,
    force: bool,
    known_hash: Option<&str>,
) -> anyhow::Result<Fetched<T>> {
    let stub = namespace.get_by_name_with_location_hint(
        &env_name.to_string(),
        &config::refresh_coordinator_location(),
    )?;

    let body = serde_json::to_string(&FetchRequest {
        env_name: env_name.clone(),
        endpoint,
        base_id: base_id.to_string(),
        ttl_ms: ttl.as_millis() as i64,
        force,
        known_hash: known_hash.map(str::to_string),
    })?;

    // The URL is only for the benefit of the logs; the request body says what we want.
    let request = Request::new_with_init(
        &format!("https://coordinator/{}", endpoint.as_str()),
        &RequestInit {
            method: Method::Post,
            body: Some(JsValue::from(body)),
            ..Default::default()
        },
    )?;

    let mut response = stub.fetch_with_request(request).await?;
    let status = response.status_code();

    if status != 200 && status != 304 {
        return Err(anyhow!(
            "The refresh coordinator responded with {}: {}",
            status,
            response.text().await.unwrap_or_default(),
        ));
    }

    let fetched_at_ms = response
        .headers()
        .get(FETCHED_AT_HEADER)?
        .and_then(|value| value.parse().ok())
        .context("The refresh coordinator's response has no valid fetch time.")?;

    if status == 304 {
        return Ok(Fetched::NotModified { fetched_at_ms });
    }

    let hash = response.headers().get(SNAPSHOT_HASH_HEADER)?;

    Ok(Fetched::Value {
        value: response.json::<T>().await?,
        fetched_at_ms,
        hash,
    })
}

// Fetch the latest data for an endpoint from NocoDB, serialized as JSON.
async fn fetch_from_noco(
    kv: &KvStore,
    env_name: &EnvName,
    base_id: &BaseId,
    endpoint: Endpoint,
) -> anyhow::Result<String> {
    let api_token = kv::get_api_token(kv, env_name)
        .await?
        .context("There is no NocoDB API token configured for this environment.")?;

    let client = NocoClient::new(url::dash_origin(env_name)?, api_token);
    let table_ids = Store::get_table_ids(kv, env_name, &client, base_id).await?;

    Ok(match endpoint {
        Endpoint::Events => serde_json::to_string(&noco::get_events(&client, &table_ids).await?)?,
        Endpoint::People => serde_json::to_string(&noco::get_people(&client, &table_ids).await?)?,
        Endpoint::Locations => {
            serde_json::to_string(&noco::get_locations(&client, &table_ids).await?)?
        }
        Endpoint::Info => serde_json::to_string(&noco::get_info(&client, &table_ids).await?)?,
        Endpoint::Pages => serde_json::to_string(&noco::get_pages(&client, &table_ids).await?)?,
        Endpoint::Announcements => {
            serde_json::to_string(&noco::get_announcements(&client, &table_ids).await?)?
        }
        Endpoint::Files => serde_json::to_string(&noco::get_files(&client, &table_ids).await?)?,
        Endpoint::Vendors => serde_json::to_string(&noco::get_vendors(&client, &table_ids).await?)?,
        Endpoint::VendorPages => {
            serde_json::to_string(&noco::get_vendor_pages(&client, &table_ids).await?)?
        }
        Endpoint::Passwords => {
            serde_json::to_string(&noco::get_passwords(&client, &table_ids).await?)?
        }
    })
}

// Bring the persistent cache up to date with a successful fetch.
//
// Writing before we hand the result back matters for announcements: once the webhook's forced
// refresh returns, it purges the edge cache, and requests that miss it read the persistent cache.
async fn write_kv(
    kv: &KvStore,
    env_name: &EnvName,
    endpoint: Endpoint,
    snapshot: &Snapshot,
    changed: bool,
) {
    let cache_key = endpoint.cache_key(env_name);

    // Even when the data hasn't changed since our last fetch, the persistent cache may have been
    // cleared in the meantime (say, by `just clear-cache`), and nothing else will put it back. So
    // we check, for the cost of one KV read per fetch. KV reads can be up to a minute out of date,
    // so it may take a few fetches to notice.
    let needs_write = changed
        || match kv::get_cached_json(kv, &cache_key).await {
            Ok(cached) => cached.as_deref() != Some(snapshot.json.as_str()),
            Err(_) => true,
        };

    if needs_write && let Err(e) = kv::put_cached_json(kv, &cache_key, &snapshot.json).await {
        log::warn(log::Message {
            id: "coordinator-persistent-cache-write-failed",
            env: env_name.into(),
            msg: &format!(
                "Coordinator failed putting `{}` in the persistent cache: {}",
                endpoint.as_str(),
                e
            ),
        })
    }
}

// The result of one fetch from NocoDB.
#[derive(Debug)]
struct Snapshot {
    json: String,
    hash: blake3::Hash,
    fetched_at_ms: i64,

    // Fetches are numbered in the order they were started, so a slow fetch that finishes late
    // can't replace data from one that started after it.
    generation: u64,
}

// Cloned for each caller waiting on the same fetch, so it has to be `Clone`; hence the `Rc` and
// the stringly-typed error.
type FetchResult = Result<Rc<Snapshot>, String>;

#[derive(Clone)]
struct InFlight {
    generation: u64,
    result: Shared<LocalBoxFuture<'static, FetchResult>>,
}

#[derive(Default)]
struct EndpointState {
    latest: Option<Rc<Snapshot>>,
    in_flight: Option<InFlight>,

    // How many fetches in a row have failed, and the time (as epoch milliseconds) before which
    // we won't try again. See `store::backoff_delay`.
    consecutive_failures: u32,
    next_attempt_at_ms: i64,
}

enum Decision {
    // What we have is younger than the TTL; hand it back.
    Serve(Rc<Snapshot>),

    // A fetch is already running; wait on it.
    Join(InFlight),

    // Fetches are in cooldown after repeated failures.
    InBackoff { remaining_ms: i64 },

    // Start a new fetch.
    Start,
}

fn decide(state: &EndpointState, now_ms: i64, ttl_ms: i64, force: bool) -> Decision {
    // A forced fetch skips the backoff too. The only caller is the announcement webhook, which
    // NocoDB itself just sent us, so that's about the best evidence we'll get that NocoDB has
    // recovered. And it happens once per announcement, so it can't pile onto an outage.
    if force {
        return Decision::Start;
    }

    if let Some(latest) = &state.latest
        && now_ms - latest.fetched_at_ms < ttl_ms
    {
        return Decision::Serve(Rc::clone(latest));
    }

    if let Some(in_flight) = &state.in_flight {
        return Decision::Join(in_flight.clone());
    }

    let remaining_ms = state.next_attempt_at_ms - now_ms;

    if remaining_ms > 0 {
        return Decision::InBackoff { remaining_ms };
    }

    Decision::Start
}

// How a fetch turned out, for the caller to act on and log.
#[derive(Debug, PartialEq, Eq)]
enum Outcome {
    // The fetch succeeded and its data is now the latest. `changed` says whether it differs from
    // what we had before, and so whether it certainly needs writing to the persistent cache.
    Updated {
        changed: bool,
    },

    // The fetch finished after a newer one had already succeeded, so its result was dropped.
    Superseded,

    // The fetch failed, and we're backing off for this long.
    Failed {
        consecutive_failures: u32,
        delay: Duration,
    },
}

// State shared between the coordinator's request handlers and the fetches they start. A fetch can
// outlive the request that started it, so this sits behind an `Rc`.
//
// A Durable Object handles requests one at a time, but it moves on to the next one whenever the
// current one awaits, so requests interleave. That's what lets concurrent callers share a fetch.
// It also means nothing here holds a `RefCell` borrow across an await.
#[derive(Default)]
struct Inner {
    endpoints: RefCell<HashMap<Endpoint, EndpointState>>,
    next_generation: Cell<u64>,
}

impl Inner {
    fn decide(&self, endpoint: Endpoint, ttl_ms: i64, force: bool) -> Decision {
        let mut endpoints = self.endpoints.borrow_mut();
        decide(
            endpoints.entry(endpoint).or_default(),
            now_ms(),
            ttl_ms,
            force,
        )
    }

    fn start_fetch(
        self: &Rc<Self>,
        kv: KvStore,
        env_name: EnvName,
        base_id: BaseId,
        endpoint: Endpoint,
    ) -> InFlight {
        let generation = self.next_generation.get();
        self.next_generation.set(generation + 1);

        let started_at_ms = now_ms();
        let inner = Rc::clone(self);

        let result = async move {
            log::log(log::Message {
                id: "coordinator-fetch-started",
                env: (&env_name).into(),
                msg: &format!(
                    "Fetching `{}` from NocoDB (generation {}).",
                    endpoint.as_str(),
                    generation
                ),
            });

            let timeout = config::noco_fetch_timeout();

            let json =
                match with_timeout(timeout, fetch_from_noco(&kv, &env_name, &base_id, endpoint))
                    .await
                {
                    Ok(Ok(json)) => Some(json),
                    Ok(Err(e)) => {
                        log::warn(log::Message {
                            id: "coordinator-fetch-failed",
                            env: (&env_name).into(),
                            msg: &format!(
                                "Failed fetching `{}` from NocoDB: {}",
                                endpoint.as_str(),
                                e
                            ),
                        });

                        None
                    }
                    Err(TimedOut) => {
                        log::warn(log::Message {
                            id: "coordinator-fetch-timeout",
                            env: (&env_name).into(),
                            msg: &format!(
                                "Fetching `{}` from NocoDB timed out after {}ms.",
                                endpoint.as_str(),
                                timeout.as_millis(),
                            ),
                        });

                        None
                    }
                };

            let (result, outcome) = inner.record(endpoint, generation, started_at_ms, json);

            match outcome {
                Outcome::Updated { changed } => {
                    if let Ok(snapshot) = &result {
                        write_kv(&kv, &env_name, endpoint, snapshot, changed).await;
                    }
                }
                Outcome::Superseded => {
                    log::log(log::Message {
                        id: "coordinator-fetch-superseded",
                        env: (&env_name).into(),
                        msg: &format!(
                            "Dropping `{}` from generation {}; a newer fetch already finished.",
                            endpoint.as_str(),
                            generation
                        ),
                    });
                }
                Outcome::Failed {
                    consecutive_failures,
                    delay,
                } => {
                    log::warn(log::Message {
                        id: "coordinator-fetch-failed",
                        env: (&env_name).into(),
                        msg: &format!(
                            "Fetching `{}` failed ({} consecutive). Backing off for {}ms.",
                            endpoint.as_str(),
                            consecutive_failures,
                            delay.as_millis(),
                        ),
                    });
                }
            }

            result
        }
        .boxed_local()
        .shared();

        let in_flight = InFlight { generation, result };

        self.endpoints
            .borrow_mut()
            .entry(endpoint)
            .or_default()
            .in_flight = Some(in_flight.clone());

        in_flight
    }

    // Record how a fetch turned out. `json` is `None` if it failed.
    fn record(
        &self,
        endpoint: Endpoint,
        generation: u64,
        started_at_ms: i64,
        json: Option<String>,
    ) -> (FetchResult, Outcome) {
        let mut endpoints = self.endpoints.borrow_mut();
        let state = endpoints.entry(endpoint).or_default();

        if state
            .in_flight
            .as_ref()
            .is_some_and(|in_flight| in_flight.generation == generation)
        {
            state.in_flight = None;
        }

        // A newer fetch has already succeeded, so this one has nothing to add, whether it
        // succeeded or failed. Hand back the newer data instead.
        if let Some(latest) = &state.latest
            && latest.generation > generation
        {
            return (Ok(Rc::clone(latest)), Outcome::Superseded);
        }

        let Some(json) = json else {
            state.consecutive_failures += 1;

            let delay = backoff_delay(state.consecutive_failures);
            state.next_attempt_at_ms = now_ms() + delay.as_millis() as i64;

            return (
                Err(format!(
                    "Fetching `{}` from NocoDB failed.",
                    endpoint.as_str()
                )),
                Outcome::Failed {
                    consecutive_failures: state.consecutive_failures,
                    delay,
                },
            );
        };

        state.consecutive_failures = 0;
        state.next_attempt_at_ms = 0;

        let hash = blake3::hash(json.as_bytes());
        let changed = state
            .latest
            .as_ref()
            .is_none_or(|latest| latest.hash != hash);

        let snapshot = Rc::new(Snapshot {
            json,
            hash,
            fetched_at_ms: started_at_ms,
            generation,
        });

        state.latest = Some(Rc::clone(&snapshot));

        (Ok(snapshot), Outcome::Updated { changed })
    }

    // Wait on a fetch. If a newer one started in the meantime (because a caller forced it), wait
    // on that one instead, so no caller comes away with data fetched before the forced refresh
    // began.
    async fn wait_for(&self, endpoint: Endpoint, mut in_flight: InFlight) -> FetchResult {
        loop {
            let result = in_flight.result.clone().await;

            let endpoints = self.endpoints.borrow();
            let state = endpoints.get(&endpoint);

            if let Some(newer) = state
                .and_then(|state| state.in_flight.as_ref())
                .filter(|newer| newer.generation > in_flight.generation)
            {
                in_flight = newer.clone();
                continue;
            }

            if let Some(latest) = state
                .and_then(|state| state.latest.as_ref())
                .filter(|latest| latest.generation > in_flight.generation)
            {
                return Ok(Rc::clone(latest));
            }

            return result;
        }
    }
}

#[durable_object(fetch)]
pub struct RefreshCoordinator {
    state: State,
    env: Env,
    inner: Rc<Inner>,
}

impl DurableObject for RefreshCoordinator {
    fn new(state: State, env: Env) -> Self {
        Self {
            state,
            env,
            inner: Rc::new(Inner::default()),
        }
    }

    async fn fetch(&self, mut req: Request) -> worker::Result<Response> {
        config::init(&self.env).map_err(|e| worker::Error::RustError(e.to_string()))?;

        let FetchRequest {
            env_name,
            endpoint,
            base_id,
            ttl_ms,
            force,
            known_hash,
        } = req.json().await?;

        let known_hash = known_hash.as_deref();

        let in_flight = match self.inner.decide(endpoint, ttl_ms, force) {
            Decision::Serve(snapshot) => return snapshot_response(&snapshot, known_hash),
            Decision::InBackoff { remaining_ms } => {
                return Response::error(
                    format!(
                        "Fetches for `{}` are in backoff for another {}ms.",
                        endpoint.as_str(),
                        remaining_ms,
                    ),
                    503,
                );
            }
            Decision::Join(in_flight) => in_flight,
            Decision::Start => {
                let in_flight = self.inner.start_fetch(
                    self.env.kv("KV")?,
                    env_name,
                    BaseId::from(base_id),
                    endpoint,
                );

                // Make sure the fetch runs to completion and records its result, even if every
                // caller waiting on it goes away.
                self.state.wait_until(in_flight.result.clone().map(|_| ()));

                in_flight
            }
        };

        match self.inner.wait_for(endpoint, in_flight).await {
            Ok(snapshot) => snapshot_response(&snapshot, known_hash),
            Err(message) => Response::error(message, 503),
        }
    }
}

// Whether the caller already has this snapshot's data, going by the hash it sent.
fn caller_has_snapshot(snapshot: &Snapshot, known_hash: Option<&str>) -> bool {
    known_hash.is_some_and(|known_hash| snapshot.hash.to_hex().as_str() == known_hash)
}

fn snapshot_response(snapshot: &Snapshot, known_hash: Option<&str>) -> worker::Result<Response> {
    let mut response = if caller_has_snapshot(snapshot, known_hash) {
        Response::empty()?.with_status(304)
    } else {
        let mut response = Response::ok(snapshot.json.clone())?;

        response
            .headers_mut()
            .set(SNAPSHOT_HASH_HEADER, snapshot.hash.to_hex().as_str())?;

        response
    };

    response
        .headers_mut()
        .set(FETCHED_AT_HEADER, &snapshot.fetched_at_ms.to_string())?;

    Ok(response)
}

#[cfg(test)]
mod tests {
    use futures::executor::block_on;
    use futures::future::ready;

    use super::*;

    const TTL_MS: i64 = 5_000;

    fn snapshot(generation: u64, fetched_at_ms: i64) -> Rc<Snapshot> {
        let json = format!("[{generation}]");

        Rc::new(Snapshot {
            hash: blake3::hash(json.as_bytes()),
            json,
            fetched_at_ms,
            generation,
        })
    }

    fn in_flight(generation: u64, result: FetchResult) -> InFlight {
        InFlight {
            generation,
            result: ready(result).boxed_local().shared(),
        }
    }

    #[test]
    fn decide_starts_a_fetch_with_nothing_cached() {
        let state = EndpointState::default();

        assert!(matches!(
            decide(&state, 10_000, TTL_MS, false),
            Decision::Start
        ));
    }

    #[test]
    fn decide_serves_data_younger_than_the_ttl() {
        let state = EndpointState {
            latest: Some(snapshot(0, 8_000)),
            in_flight: Some(in_flight(1, Err(String::new()))),
            ..Default::default()
        };

        assert!(matches!(
            decide(&state, 10_000, TTL_MS, false),
            Decision::Serve(latest) if latest.generation == 0
        ));
    }

    #[test]
    fn decide_joins_a_running_fetch_once_data_is_older_than_the_ttl() {
        let state = EndpointState {
            latest: Some(snapshot(0, 1_000)),
            in_flight: Some(in_flight(1, Err(String::new()))),
            ..Default::default()
        };

        assert!(matches!(
            decide(&state, 10_000, TTL_MS, false),
            Decision::Join(in_flight) if in_flight.generation == 1
        ));
    }

    #[test]
    fn decide_force_skips_fresh_data_and_running_fetches() {
        let state = EndpointState {
            latest: Some(snapshot(0, 9_000)),
            in_flight: Some(in_flight(1, Err(String::new()))),
            ..Default::default()
        };

        assert!(matches!(
            decide(&state, 10_000, TTL_MS, true),
            Decision::Start
        ));
    }

    #[test]
    fn decide_respects_backoff_unless_forced() {
        let state = EndpointState {
            latest: Some(snapshot(0, 1_000)),
            consecutive_failures: 2,
            next_attempt_at_ms: 12_000,
            ..Default::default()
        };

        assert!(matches!(
            decide(&state, 10_000, TTL_MS, false),
            Decision::InBackoff {
                remaining_ms: 2_000
            }
        ));

        assert!(matches!(
            decide(&state, 10_000, TTL_MS, true),
            Decision::Start
        ));
    }

    #[test]
    fn record_skips_the_kv_write_when_nothing_changed() {
        let inner = Inner::default();

        let (_, first) = inner.record(Endpoint::Events, 0, 1_000, Some("[1]".into()));
        let (_, second) = inner.record(Endpoint::Events, 1, 6_000, Some("[1]".into()));
        let (_, third) = inner.record(Endpoint::Events, 2, 11_000, Some("[2]".into()));

        assert_eq!(first, Outcome::Updated { changed: true });
        assert_eq!(second, Outcome::Updated { changed: false });
        assert_eq!(third, Outcome::Updated { changed: true });
    }

    #[test]
    fn record_drops_a_fetch_that_finishes_after_a_newer_one() {
        let inner = Inner::default();

        let _ = inner.record(Endpoint::Announcements, 1, 2_000, Some("[\"new\"]".into()));
        let (result, outcome) =
            inner.record(Endpoint::Announcements, 0, 1_000, Some("[\"old\"]".into()));

        assert_eq!(outcome, Outcome::Superseded);
        assert_eq!(result.unwrap().json, "[\"new\"]");

        let endpoints = inner.endpoints.borrow();
        let latest = endpoints[&Endpoint::Announcements].latest.as_ref().unwrap();
        assert_eq!(latest.generation, 1);
    }

    #[test]
    fn record_backs_off_on_failure_and_resets_on_success() {
        let inner = Inner::default();

        let (result, outcome) = inner.record(Endpoint::People, 0, 1_000, None);
        assert!(result.is_err());
        assert!(matches!(
            outcome,
            Outcome::Failed {
                consecutive_failures: 1,
                ..
            }
        ));
        assert!(inner.endpoints.borrow()[&Endpoint::People].next_attempt_at_ms > 0);

        let _ = inner.record(Endpoint::People, 1, 2_000, Some("[]".into()));

        let endpoints = inner.endpoints.borrow();
        let state = &endpoints[&Endpoint::People];
        assert_eq!(state.consecutive_failures, 0);
        assert_eq!(state.next_attempt_at_ms, 0);
    }

    #[test]
    fn record_ignores_a_failure_from_a_superseded_fetch() {
        let inner = Inner::default();

        let _ = inner.record(Endpoint::Pages, 1, 2_000, Some("[]".into()));
        let (result, outcome) = inner.record(Endpoint::Pages, 0, 1_000, None);

        assert_eq!(outcome, Outcome::Superseded);
        assert!(result.is_ok());
        assert_eq!(
            inner.endpoints.borrow()[&Endpoint::Pages].consecutive_failures,
            0
        );
    }

    #[test]
    fn caller_has_snapshot_only_when_the_hash_matches() {
        let latest = snapshot(1, 1_000);
        let latest_hash = latest.hash.to_hex();
        let older_hash = snapshot(0, 0).hash.to_hex();

        assert!(caller_has_snapshot(&latest, Some(latest_hash.as_str())));
        assert!(!caller_has_snapshot(&latest, Some(older_hash.as_str())));
        assert!(!caller_has_snapshot(&latest, None));
    }

    #[test]
    fn fetch_request_without_known_hash_still_parses() {
        // What a worker deployed before conditional refreshes sends.
        let request: FetchRequest = serde_json::from_str(
            r#"{"env_name":"mycon","endpoint":"events","base_id":"b","ttl_ms":5000,"force":false}"#,
        )
        .unwrap();

        assert_eq!(request.known_hash, None);
    }

    #[test]
    fn wait_for_follows_a_newer_fetch() {
        let inner = Inner::default();
        let newer = in_flight(1, Ok(snapshot(1, 2_000)));

        inner
            .endpoints
            .borrow_mut()
            .entry(Endpoint::Announcements)
            .or_default()
            .in_flight = Some(newer);

        let older = in_flight(0, Ok(snapshot(0, 1_000)));
        let result = block_on(inner.wait_for(Endpoint::Announcements, older)).unwrap();

        assert_eq!(result.generation, 1);
    }

    #[test]
    fn wait_for_prefers_newer_data_that_already_landed() {
        let inner = Inner::default();
        let _ = inner.record(Endpoint::Info, 1, 2_000, Some("{}".into()));

        let older = in_flight(0, Err("timed out".into()));
        let result = block_on(inner.wait_for(Endpoint::Info, older)).unwrap();

        assert_eq!(result.generation, 1);
    }

    #[test]
    fn wait_for_returns_its_own_result_when_nothing_is_newer() {
        let inner = Inner::default();
        let own = in_flight(0, Err("timed out".into()));

        assert!(block_on(inner.wait_for(Endpoint::Files, own)).is_err());
    }
}
