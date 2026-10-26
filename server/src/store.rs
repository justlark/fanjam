use std::collections::HashMap;
use std::fmt;
use std::future::Future;
use std::pin::pin;
use std::sync::{Arc, Mutex, OnceLock};
use std::time::Duration;

use axum::{
    body::Body,
    http::{self, Uri},
    response::IntoResponse,
};
use futures::future::{Either, FutureExt, LocalBoxFuture, select};
use rand::Rng;
use serde::{Serialize, de::IgnoredAny};
use worker::kv::KvStore;
use worker::{Cache, Context, Delay, ObjectNamespace, console_error, console_log, console_warn};

use crate::api::{DataResponseEnvelope, Freshness, PostBackupKind};
use crate::cache::{EtagJson, put_data_cdn_cache};
use crate::coordinator::{self, Endpoint};
use crate::env::{Config, EnvId, EnvName};
use crate::error::Error;
use crate::neon::BackupSnapshot;
use crate::noco::{self, BaseId, ExistingMigrationState, MigrationState, TableIds};
use crate::router::AppState;
use crate::{cf, config, kv, url};
use crate::{
    neon::Client as NeonClient,
    noco::Client as NocoClient,
    sql::{Client as DbClient, ConnectionConfig as DbConnectionConfig},
};

/// Per-endpoint bookkeeping for background refreshes, scoped to this isolate and keyed by the edge
/// cache URL (see `refresh_key`). It does two jobs:
///
/// 1. **Stampede protection.** If a refresh is already in flight for a key, we do not spawn
///    another. Otherwise every request arriving in the window after the edge cache expires would
///    start one.
///
/// 2. **Backoff.** When a refresh fails, we record a cooldown before another may be attempted,
///    growing exponentially with consecutive failures. Without this, a fast-failing upstream gets
///    re-attempted at request arrival rate.
///
/// This only works **within this isolate**, and under heavy load, Cloudflare runs a great many of
/// them. Each one refreshing on its own was enough to knock NocoDB over in load testing, so a
/// refresh doesn't go to NocoDB directly; it goes to the refresh coordinator (see `coordinator`),
/// which fetches at most once per TTL for every isolate in every datacenter, and has its own
/// backoff covering all of them. What's left here limits each isolate to one call to the
/// coordinator at a time, and backs it off when the coordinator says NocoDB is failing.
///
/// Workers isolates are inherently single-threaded, so the Mutex never actually contends; it's
/// only needed to satisfy Send/Sync bounds.
static REFRESH_STATE: OnceLock<Mutex<HashMap<String, RefreshState>>> = OnceLock::new();

fn refresh_state() -> &'static Mutex<HashMap<String, RefreshState>> {
    REFRESH_STATE.get_or_init(|| Mutex::new(HashMap::new()))
}

#[derive(Debug, Default)]
struct RefreshState {
    // Whether a background refresh for this key is currently running.
    in_flight: bool,

    // How many times in a row the refresh has failed. Reset on success.
    consecutive_failures: u32,

    // Wall-clock milliseconds before which no new refresh may be attempted.
    next_attempt_at_ms: i64,
}

// The cooldown imposed after a single failed refresh. Each consecutive failure doubles it.
const BASE_BACKOFF: Duration = Duration::from_secs(1);

// The ceiling on the cooldown, so that even a long outage still gets probed periodically and the
// system recovers on its own once NocoDB comes back.
const MAX_BACKOFF: Duration = Duration::from_secs(60);

// How long a background refresh may run before we abandon it and count it as a failure. Generous,
// because a NocoDB instance waking from suspend is legitimately slow; the point is only to stop a
// hung fetch from holding the in-flight guard indefinitely.
//
// This is a little longer than the coordinator's own timeout on NocoDB, so a slow fetch times out
// there, where its backoff covers every isolate, rather than here.
fn refresh_timeout() -> Duration {
    config::noco_fetch_timeout() + Duration::from_secs(5)
}

// The key for `REFRESH_STATE`. The edge cache URL pins down both the environment and the endpoint,
// and unlike the environment name, it's known on an edge cache hit without any KV reads.
fn refresh_key(uri: &Uri) -> String {
    uri.to_string()
}

// What to do about refreshing a given cache key *right now*.
//
// Note that this is a separate question from how to classify the freshness of the response we're
// about to serve, and the two must not be conflated. Whether to spawn a refresh depends on what
// happens to be happening this instant; whether the client should check back depends on whether
// upstream is healthy at all. When NocoDB is down, a refresh is often in flight — but it's a
// doomed probe, not an imminent update, and there is nothing for the client to come back for.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RefreshAction {
    // Nothing is running and we're not in cooldown: spawn a refresh.
    Spawn,

    // A refresh is already running; don't spawn a second one.
    AlreadyInFlight,

    // Upstream is in cooldown after repeated failures; don't attempt one.
    InBackoff { remaining_ms: i64 },
}

// Wall-clock milliseconds. Not monotonic, but Workers freezes the clock within a request and only
// advances it after I/O, and every decision point here follows a network await, so it advances
// when we need it to.
pub(crate) fn now_ms() -> i64 {
    chrono::Utc::now().timestamp_millis()
}

// How long to wait before attempting another refresh, after `consecutive_failures` failures in a
// row. Doubles per failure up to `MAX_BACKOFF`, with equal jitter (half the delay fixed, half
// random) so isolates that failed at the same moment don't all retry in lockstep.
pub(crate) fn backoff_delay(consecutive_failures: u32) -> Duration {
    // Capping the exponent keeps the multiplication well clear of overflow; anything past a few
    // failures is clamped to `MAX_BACKOFF` regardless.
    let exponent = consecutive_failures.saturating_sub(1).min(16);
    let scaled = (BASE_BACKOFF * 2u32.saturating_pow(exponent)).min(MAX_BACKOFF);
    let half = scaled / 2;

    half + half.mul_f64(rand::thread_rng().gen_range(0.0..1.0))
}

// Decide whether to spawn a background refresh for `key`, marking it in flight if so, and classify
// how current the data we're about to serve from the persistent cache is. The caller must call
// `finish_refresh` when the refresh completes, but only if this returned `RefreshAction::Spawn`.
fn try_begin_refresh(key: &str) -> (RefreshAction, Freshness) {
    let mut state = refresh_state().lock().unwrap();
    let entry = state.entry(key.to_string()).or_default();

    // Freshness comes off the failure count, *not* off the action below. Any recorded failure
    // means upstream is unhealthy, and telling the client to check back would only add to the load
    // keeping it down — regardless of whether we happen to have a probe in flight at this instant.
    // Before the first failure we have no evidence of trouble, so `Stale` is the honest answer.
    let freshness = if entry.consecutive_failures > 0 {
        Freshness::Backoff
    } else {
        Freshness::Stale
    };

    if entry.in_flight {
        return (RefreshAction::AlreadyInFlight, freshness);
    }

    let remaining_ms = entry.next_attempt_at_ms - now_ms();

    if remaining_ms > 0 {
        return (RefreshAction::InBackoff { remaining_ms }, freshness);
    }

    entry.in_flight = true;

    (RefreshAction::Spawn, freshness)
}

// How much of the cooldown for `key` is left, if it's currently in one. Used on the blocking path,
// where there's no cached value to fall back on: if upstream is in cooldown, there's no point
// making the caller wait on a request we expect to fail.
fn backoff_remaining_ms(key: &str) -> Option<i64> {
    let state = refresh_state().lock().unwrap();
    let entry = state.get(key)?;
    let remaining_ms = entry.next_attempt_at_ms - now_ms();

    (remaining_ms > 0).then_some(remaining_ms)
}

// Record the outcome of an upstream fetch, clearing the in-flight flag and either resetting or
// extending the cooldown. Used by both the background refresh and the blocking cold-cache path;
// the latter never sets the in-flight flag, but clearing it again is harmless.
fn finish_refresh(key: &str, succeeded: bool) {
    let mut state = refresh_state().lock().unwrap();
    let entry = state.entry(key.to_string()).or_default();

    entry.in_flight = false;

    if succeeded {
        let recovered_from = entry.consecutive_failures;

        // A healthy key needs no state, and dropping it bounds the size of the map.
        state.remove(key);

        if recovered_from > 0 {
            console_log!(
                "Refresh for {} recovered after {} consecutive failures.",
                key,
                recovered_from,
            );
        }

        return;
    }

    entry.consecutive_failures += 1;

    let delay = backoff_delay(entry.consecutive_failures);
    entry.next_attempt_at_ms = now_ms() + delay.as_millis() as i64;

    console_warn!(
        "Refresh for {} failed ({} consecutive). Backing off for {}ms.",
        key,
        entry.consecutive_failures,
        delay.as_millis(),
    );
}

// Run an upstream request, giving up after `timeout`. Dropping the losing future cancels the
// in-flight `fetch`, and `Delay` cancels its timer when dropped, so neither leaks.
pub(crate) async fn with_timeout<F, T>(key: &str, timeout: Duration, request: F) -> Option<T>
where
    F: Future<Output = Option<T>>,
{
    let request = pin!(request);
    let delay = pin!(Delay::from(timeout));

    match select(request, delay).await {
        Either::Left((value, _)) => value,
        Either::Right(((), _)) => {
            console_warn!(
                "Refresh for {} timed out after {}s.",
                key,
                timeout.as_secs()
            );

            None
        }
    }
}

async fn with_refresh_timeout<F, T>(key: &str, request: F) -> Option<T>
where
    F: Future<Output = Option<T>>,
{
    with_timeout(key, refresh_timeout(), request).await
}

// Writes freshly fetched data to both the persistent cache and the edge cache.
pub type CacheWrite = LocalBoxFuture<'static, ()>;

// Run a refresh in the background, write the result to both caches, and record the outcome. Only
// call this after `try_begin_refresh` returned `RefreshAction::Spawn` for `key`.
//
// The in-flight flag stays set until the caches have been written, not just until NocoDB answers.
// Clearing it any sooner would let a request arriving in between, which still sees the old edge
// cache entry, spawn a second refresh.
fn spawn_refresh<T, R>(ctx: &Context, key: String, refresh: R)
where
    T: 'static,
    R: Future<Output = Option<(T, CacheWrite)>> + 'static,
{
    ctx.wait_until(async move {
        let latest = with_refresh_timeout(&key, refresh).await;
        let succeeded = latest.is_some();

        if let Some((_, write_cache)) = latest {
            write_cache.await;
        }

        finish_refresh(&key, succeeded);
    });
}

// Called on an edge cache hit that's due for a refresh (see `cache::put_data_cdn_cache`). This
// starts a background refresh to replace it, subject to the same in-flight guard and backoff as a
// refresh from the persistent cache, and returns the freshness to mark the hit with, classified
// the same way too.
//
// `refresh` is handed the `Store` for the environment, which we only build once we know a refresh
// is going ahead. Building it costs several KV reads, and most hits on a due entry arrive while a
// refresh is already running.
pub fn refresh_ahead<T, F, R>(
    state: &Arc<AppState>,
    env_id: EnvId,
    uri: Uri,
    refresh: F,
) -> Freshness
where
    T: 'static,
    F: FnOnce(&Store, Uri) -> R + 'static,
    R: Future<Output = Option<(T, CacheWrite)>> + 'static,
{
    let key = refresh_key(&uri);
    let (action, freshness) = try_begin_refresh(&key);

    // This is the hot path, so unlike the persistent-cache path, we don't log when we skip.
    if action != RefreshAction::Spawn {
        return freshness;
    }

    let state_for_refresh = Arc::clone(state);

    let refresh = async move {
        match Store::from_env_id(&state_for_refresh, &env_id).await {
            Ok(store) => refresh(&store, uri).await,
            Err(e) => {
                console_warn!("Failed setting up refresh for {}: {}", uri, e);
                None
            }
        }
    };

    spawn_refresh(&state.ctx, key, refresh);

    freshness
}

pub struct Store {
    noco_client: NocoClient,
    neon_client: NeonClient,
    kv: KvStore,
    coordinator: ObjectNamespace,
    ctx: Arc<Context>,
    env_name: EnvName,
    base_id: BaseId,
    env_config: Config,
}

impl fmt::Debug for Store {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Migrator")
            .field("noco_client", &self.noco_client)
            .field("neon_client", &self.neon_client)
            .finish_non_exhaustive()
    }
}

#[derive(Debug)]
pub struct MigrationChange {
    pub old_version: noco::Version,
    pub new_version: noco::Version,
}

// This macro generates methods on the `Store` for fetching data from NocoDB with caching.
//
// We're using the Cloudflare cache API with a short TTL (configurable per-environment, but likely
// on the order of seconds) to reduce the load on the upstream NocoDB instance. We'll call this the
// "edge cache". Using this edge cache means incoming requests will only be routed upstream at most
// once every *n* milliseconds per data center per cache key. This is important, because the NocoDB
// instance is slower and much more expensive to scale than this worker.
//
// Edge cache entries are refreshed *ahead* of expiry. An entry is kept for a grace period past its
// TTL, and a hit after the TTL is still served, but also kicks off a background refresh (see
// `cache::put_data_cdn_cache` and `refresh_ahead`). So as long as requests keep coming in, the edge
// cache never actually expires, and requests never have to wait on NocoDB or go anywhere else.
// Those hits are marked `Stale` (see below), so the client still checks back for the refreshed
// data; that retry is just another cheap edge cache hit.
//
// When traffic is quiet for long enough that an entry does expire, or after it's purged, we need
// to fetch fresh data from NocoDB. However, this runs the risk of a cache stampede, where many
// requests arriving all at once hammer the upstream NocoDB instance. To remedy this, we have a
// second cache stored in KV that never expires. We'll call this the "persistent cache". Requests
// that miss the edge cache will hit the persistent cache instead, which will then kick off a
// background task (that does not block the request from returning) to update both caches with
// fresh data from NocoDB.
//
// Both kinds of background refresh share a locking mechanism to ensure that only one request at a
// time can trigger them, otherwise we would have the same cache stampede problem. See
// `REFRESH_STATE`. That only covers one isolate, though, so refreshes don't go to NocoDB directly.
// They go through the refresh coordinator, which fetches from NocoDB at most once per TTL for
// everyone, and keeps the persistent cache up to date itself. See `coordinator`.
//
// Every response carries a `Freshness` telling the client how current the data is. When we serve
// out of the persistent cache and NocoDB is healthy, we mark it `Stale`, and the client checks
// back on its own schedule to pick up the fresher data. Once a refresh has failed, we mark it
// `Backoff` instead and the client does not retry at all — retrying would only add to the load
// that's keeping NocoDB down.
//
// Note that `Backoff` is keyed off whether upstream has been failing, *not* off whether we happen
// to have a refresh in flight at this instant. During an outage a doomed probe is in flight much
// of the time, and treating that as "fresher data is on its way" would tell the client to come
// back for something that isn't coming.
macro_rules! get_data {
    {
        fn_name: $fn_name:ident,
        refresh_fn_name: $refresh_fn_name:ident,
        type_name: $type_name:ty,
        endpoint: $endpoint:expr,
        get_api_fn: $get_api_fn:path,
        get_cached_fn: $get_cached_fn:path,
        put_cached_fn: $put_cached_fn:path,
        cache_key: $cache_key:expr,
    } => {
        // Get the latest data, by way of the refresh coordinator. On success, this resolves to the
        // response body, along with a future that writes the data to the edge cache; the caller
        // decides whether to await that or hand it to `Context::wait_until`.
        //
        // The returned future owns everything it needs, so it can outlive the request.
        pub fn $refresh_fn_name<T, F>(
            &self,
            uri: Uri,
            to_body: F,
        ) -> impl Future<Output = Option<(T, CacheWrite)>> + use<T, F>
        where
            T: Serialize + Clone + 'static,
            F: FnOnce($type_name) -> T + 'static,
        {
            let kv = self.kv.clone();
            let env_name = self.env_name.clone();
            let noco_client = self.noco_client.clone();
            let base_id = self.base_id.clone();
            let coordinator = self.coordinator.clone();
            let cache_ttl = self.cache_ttl();

            async move {
                // The coordinator keeps the persistent cache up to date itself. The kill switch
                // falls back to fetching from NocoDB directly, the way every isolate used to, and
                // then it's on us to write the persistent cache too.
                let (value, fetched_at_ms, value_for_kv): ($type_name, i64, Option<$type_name>) =
                    if config::use_refresh_coordinator() {
                        let result = coordinator::fetch::<$type_name>(
                            &coordinator,
                            &env_name,
                            &base_id,
                            $endpoint,
                            cache_ttl,
                            false,
                        )
                        .await;

                        match result {
                            Ok(fetched) => (fetched.value, fetched.fetched_at_ms, None),
                            Err(e) => {
                                console_warn!(
                                    "Failed getting {} from the refresh coordinator: {}",
                                    $cache_key,
                                    e,
                                );
                                return None;
                            }
                        }
                    } else {
                        let fetched_at_ms = now_ms();

                        let table_ids = match Self::get_table_ids(&kv, &env_name, &noco_client, &base_id).await {
                            Ok(table_ids) => table_ids,
                            Err(e) => {
                                console_warn!("Failed getting table IDs from NocoDB: {}", e);
                                return None;
                            }
                        };

                        match $get_api_fn(&noco_client, &table_ids).await {
                            Ok(value) => {
                                let value_for_kv = value.clone();
                                (value, fetched_at_ms, Some(value_for_kv))
                            }
                            Err(e) => {
                                console_warn!("Failed getting {} from NocoDB: {}", $cache_key, e);
                                return None;
                            }
                        }
                    };

                let body = to_body(value);
                let body_for_cache = body.clone();

                let write_cache = async move {
                    console_log!("Caching latest {} in the edge cache.", $cache_key);

                    if let Some(value) = value_for_kv
                        && let Err(e) = $put_cached_fn(&kv, &env_name, &value).await
                    {
                        console_warn!("Failed putting {} in KV cache: {}", $cache_key, e);
                    }

                    // Responses that hit the edge cache are by definition fresh; this entry is
                    // only written after a successful refresh. Marking it otherwise would catch
                    // the client in an infinite retry loop.
                    //
                    // Hits past the TTL are the exception, and get re-marked on the way out. See
                    // `cache::with_freshness`.
                    let response_for_edge_cache_result = worker::Response::try_from(
                        EtagJson(DataResponseEnvelope::new(Freshness::Fresh, body_for_cache))
                        .into_response()
                    );

                    let response_for_edge_cache = match response_for_edge_cache_result {
                        Ok(response) => response,
                        Err(_) => return,
                    };

                    put_data_cdn_cache(
                        &Cache::default(),
                        env_name,
                        cache_ttl,
                        fetched_at_ms,
                        uri,
                        response_for_edge_cache,
                    ).await;
                };

                Some((body, write_cache.boxed_local()))
            }
        }

        // Serve data on an edge cache miss: out of the persistent cache if it has anything,
        // otherwise straight from NocoDB.
        #[worker::send]
        pub async fn $fn_name<T, F>(&self, uri: Uri, to_body: F) -> Result<http::Response<Body>, Error>
        where
            T: Serialize + Clone + 'static,
            F: FnOnce($type_name) -> T + Clone + 'static,
        {

            let cached_value = match $get_cached_fn(&self.kv, &self.env_name).await {
                Ok(Some(value)) => {
                    console_log!("Returning stale {} from cache.", $cache_key);
                    Some(value)
                }
                Ok(None) => {
                    None
                }
                Err(_) => {
                    console_warn!("Failed getting cached {} from KV.", $cache_key);
                    None
                }
            };

            let refresh_key = refresh_key(&uri);

            match cached_value {
                Some(cached_value) => {
                    let (action, freshness) = try_begin_refresh(&refresh_key);

                    match action {
                        RefreshAction::Spawn => {
                            let refresh = self.$refresh_fn_name(uri, to_body.clone());
                            spawn_refresh(&self.ctx, refresh_key, refresh);
                        }
                        RefreshAction::AlreadyInFlight => {
                            console_log!(
                                "Skipping background refresh for {} (already in flight).",
                                $cache_key,
                            );
                        }
                        RefreshAction::InBackoff { remaining_ms } => {
                            console_log!(
                                "Skipping background refresh for {} ({}ms of backoff remaining).",
                                $cache_key,
                                remaining_ms,
                            );
                        }
                    }

                    let body = to_body(cached_value);

                    Ok(EtagJson(DataResponseEnvelope::new(freshness, body)).into_response())
                },
                None => {
                    // The persistent cache is empty, which should only be the case for new
                    // environments or after the cache is manually cleared. We need to block and
                    // wait for the upstream request, because we have nothing else to serve.

                    // If upstream is in cooldown, we would only be making the caller wait on a
                    // request we expect to fail. Fail fast instead of piling onto the load that's
                    // keeping NocoDB down; the response is a 503 either way.
                    if let Some(remaining_ms) = backoff_remaining_ms(&refresh_key) {
                        console_warn!(
                            "Not fetching {} from NocoDB: the persistent cache is empty and \
                             upstream is in backoff for another {}ms.",
                            $cache_key,
                            remaining_ms,
                        );

                        return Err(Error::NocoUnavailable);
                    }

                    // Deliberately no in-flight guard and no timeout of our own on this path. A
                    // legitimate cold start — a new environment, or one whose cache was just
                    // cleared — has to be allowed to block and fetch. Every request here waits on
                    // the coordinator, which makes them all share one fetch from NocoDB.
                    let latest = self.$refresh_fn_name(uri, to_body).await;

                    finish_refresh(&refresh_key, latest.is_some());

                    if let Some((body, write_cache)) = latest {
                        self.ctx.wait_until(write_cache);

                        // This data came straight from NocoDB, so it's as current as it gets. The
                        // edge cache is populated in the background above, so there's nothing for
                        // the client to check back for.
                        Ok(EtagJson(DataResponseEnvelope::new(Freshness::Fresh, body))
                            .into_response())
                    } else {
                        Err(Error::NocoUnavailable)
                    }
                },
            }
        }
    }
}

impl Store {
    pub async fn from_env_name(state: &AppState, env_name: EnvName) -> Result<Self, Error> {
        let kv = state.kv.clone();
        let ctx = Arc::clone(&state.ctx);

        let api_token = kv::get_api_token(&kv, &env_name)
            .await
            .map_err(Error::Internal)?
            .ok_or(Error::NoApiToken)?;

        let env_config = kv::get_env_config(&kv, &env_name)
            .await
            .map_err(Error::Internal)?;

        // The Postgres database is the source of truth for the base ID, but we cache it in KV to
        // avoid needing to open a separate Postgres connection per request. Otherwise, heavy load
        // would exhaust the connection pool and cause this worker to start returning 500 errors.
        let base_id = match kv::get_base_id(&kv, &env_name)
            .await
            .map_err(Error::Internal)?
        {
            Some(base_id) => base_id,
            None => {
                let db_client = DbClient::connect(
                    &Option::<DbConnectionConfig>::from(env_config.clone())
                        .ok_or(Error::MissingEnvConfig)?,
                )
                .await
                .map_err(Error::Internal)?;

                let base_id = db_client
                    .get_base()
                    .await
                    .map_err(Error::Internal)?
                    .ok_or(Error::NoBaseId)?;

                if let Err(e) = kv::put_base_id(&kv, &env_name, &base_id).await {
                    // This need not be a fatal error.
                    console_warn!("Failed to cache base_id in KV: {}", e);
                }

                base_id
            }
        };

        let dash_origin = url::dash_origin(&env_name).map_err(Error::Internal)?;

        let noco_client = NocoClient::new(dash_origin.clone(), api_token);
        let neon_client = NeonClient::new();

        Ok(Self {
            noco_client,
            neon_client,
            kv,
            coordinator: state.coordinator.clone(),
            ctx,
            env_name,
            base_id,
            env_config,
        })
    }

    async fn connect_db(&self) -> Result<DbClient, Error> {
        DbClient::connect(
            &Option::<DbConnectionConfig>::from(self.env_config.clone())
                .ok_or(Error::MissingEnvConfig)?,
        )
        .await
        .map_err(Error::Internal)
    }

    pub async fn from_env_id(state: &AppState, env_id: &EnvId) -> Result<Self, Error> {
        let env_name = kv::get_id_env(&state.kv, env_id)
            .await
            .map_err(Error::Internal)?
            .ok_or(Error::NoEnvId)?;

        Self::from_env_name(state, env_name).await
    }

    pub fn env_name(&self) -> &EnvName {
        &self.env_name
    }

    fn cache_ttl(&self) -> Duration {
        crate::cache::cache_ttl(&self.env_config)
    }

    pub(crate) async fn get_table_ids(
        kv: &KvStore,
        env_name: &EnvName,
        noco_client: &NocoClient,
        base_id: &BaseId,
    ) -> Result<TableIds, Error> {
        Ok(
            match kv::get_tables(kv, env_name)
                .await
                .and_then(TableIds::try_from)
            {
                Ok(table_ids) => table_ids,
                Err(e) => {
                    console_log!("Failed to get table IDs from KV: {}", e);

                    let table_ids = noco::list_tables(noco_client, base_id)
                        .await
                        .map_err(Error::Internal)?;

                    kv::put_tables(kv, env_name, &table_ids)
                        .await
                        .map_err(Error::Internal)?;

                    table_ids.try_into().map_err(Error::Internal)?
                }
            },
        )
    }

    get_data! {
        fn_name: get_events,
        refresh_fn_name: refresh_events,
        type_name: Vec<noco::Event>,
        endpoint: Endpoint::Events,
        get_api_fn: noco::get_events,
        get_cached_fn: kv::get_cached_events,
        put_cached_fn: kv::put_cached_events,
        cache_key: "events",
    }

    get_data! {
        fn_name: get_people,
        refresh_fn_name: refresh_people,
        type_name: Vec<noco::Person>,
        endpoint: Endpoint::People,
        get_api_fn: noco::get_people,
        get_cached_fn: kv::get_cached_people,
        put_cached_fn: kv::put_cached_people,
        cache_key: "people",
    }

    get_data! {
        fn_name: get_locations,
        refresh_fn_name: refresh_locations,
        type_name: Vec<noco::Location>,
        endpoint: Endpoint::Locations,
        get_api_fn: noco::get_locations,
        get_cached_fn: kv::get_cached_locations,
        put_cached_fn: kv::put_cached_locations,
        cache_key: "locations",
    }

    get_data! {
        fn_name: get_info,
        refresh_fn_name: refresh_info,
        type_name: noco::Info,
        endpoint: Endpoint::Info,
        get_api_fn: noco::get_info,
        get_cached_fn: kv::get_cached_info,
        put_cached_fn: kv::put_cached_info,
        cache_key: "info",
    }

    get_data! {
        fn_name: get_pages,
        refresh_fn_name: refresh_pages,
        type_name: Vec<noco::Page>,
        endpoint: Endpoint::Pages,
        get_api_fn: noco::get_pages,
        get_cached_fn: kv::get_cached_pages,
        put_cached_fn: kv::put_cached_pages,
        cache_key: "pages",
    }

    get_data! {
        fn_name: get_announcements,
        refresh_fn_name: refresh_announcements,
        type_name: Vec<noco::Announcement>,
        endpoint: Endpoint::Announcements,
        get_api_fn: noco::get_announcements,
        get_cached_fn: kv::get_cached_announcements,
        put_cached_fn: kv::put_cached_announcements,
        cache_key: "announcements",
    }

    get_data! {
        fn_name: get_files,
        refresh_fn_name: refresh_files,
        type_name: Vec<noco::File>,
        endpoint: Endpoint::Files,
        get_api_fn: noco::get_files,
        get_cached_fn: kv::get_cached_files,
        put_cached_fn: kv::put_cached_files,
        cache_key: "files",
    }

    // Refresh the cache specifically with the latest announcements from NocoDB. This is necessary
    // because we send out push notifications for announcements.
    #[worker::send]
    pub async fn refresh_announcements_cache(&self, env_id: &EnvId) -> Result<(), Error> {
        if config::use_refresh_coordinator() {
            // Force a fetch, since the coordinator's latest data may well predate the new
            // announcement. The coordinator writes the persistent cache before it answers, and we
            // don't need the data itself.
            coordinator::fetch::<IgnoredAny>(
                &self.coordinator,
                &self.env_name,
                &self.base_id,
                Endpoint::Announcements,
                self.cache_ttl(),
                true,
            )
            .await
            .map_err(Error::Internal)?;
        } else {
            let table_ids =
                Self::get_table_ids(&self.kv, &self.env_name, &self.noco_client, &self.base_id)
                    .await?;

            let announcements = noco::get_announcements(&self.noco_client, &table_ids)
                .await
                .map_err(Error::Internal)?;

            // Refresh the persistent cache.
            kv::put_cached_announcements(&self.kv, &self.env_name, &announcements)
                .await
                .map_err(Error::Internal)?;
        }

        // Purge the edge cache for the announcements endpoint so incoming requests hit the
        // persistent cache. We only purge this endpoint--not the whole environment.
        let announcements_url = url::announcements_api_url(env_id).map_err(Error::Internal)?;

        cf::Client::new()
            .purge_cache_urls(&config::cloudflare_zone_id(), &[announcements_url])
            .await
            .map_err(Error::Internal)?;

        Ok(())
    }

    #[worker::send]
    pub async fn create_backup(&self, kind: PostBackupKind) -> Result<(), Error> {
        let backup_branch = match kind {
            PostBackupKind::Deployment => BackupSnapshot::Deployment,
        };

        let project_id = self
            .neon_client
            .lookup_project(&self.env_name.clone().into())
            .await
            .map_err(Error::Internal)?;

        self.neon_client
            .create_backup(&project_id, backup_branch)
            .await
            .map_err(Error::Internal)?;

        Ok(())
    }

    pub async fn migrate(&self) -> Result<MigrationChange, Error> {
        let db_client = self.connect_db().await?;

        let old_version = db_client
            .get_current_migration()
            .await
            .map_err(Error::Internal)?;

        let migration_state = MigrationState::existing(old_version, self.base_id.clone());

        let env_id = kv::get_env_id(&self.kv, &self.env_name)
            .await
            .map_err(Error::Internal)?;

        let migrator = noco::Migrator::new(&self.noco_client, &self.neon_client, &db_client);

        let ExistingMigrationState {
            version: new_version,
            ..
        } = migrator
            .migrate(&self.env_name, env_id, migration_state)
            .await
            .map_err(Error::Internal)?;

        Ok(MigrationChange {
            old_version,
            new_version,
        })
    }

    #[worker::send]
    pub async fn delete_base(&self) -> Result<(), Error> {
        let project_id = self
            .neon_client
            .lookup_project(&self.env_name.clone().into())
            .await
            .map_err(Error::Internal)?;

        // Back up the database in case we delete the NocoDB base accidentally.
        self.neon_client
            .create_backup(&project_id, BackupSnapshot::BaseDeletion)
            .await
            .map_err(Error::Internal)?;

        // Do this first, in case the deletion fails and we need to roll back. Deleting the cache
        // is non-destructive, but *not* deleting the cache after we've deleted the base would
        // leave the environment in an inconsistent state.
        kv::delete_cache(&self.kv, &self.env_name)
            .await
            .map_err(Error::Internal)?;

        let db_client = self.connect_db().await?;

        self.neon_client
            .with_rollback(&self.env_name, async || {
                let result = {
                    noco::delete_base(&self.noco_client, &self.base_id).await?;
                    db_client.delete_base(&self.base_id).await?;
                    Ok(())
                };

                match result {
                    Err(e) => {
                        console_error!("{:?}", e);
                        console_error!("Failed deleting base. Rolling back.");
                        Err(e)
                    }
                    Ok(_) => Ok(()),
                }
            })
            .await
            .map_err(Error::Internal)?;

        Ok(())
    }
}
