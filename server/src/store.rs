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
use futures::future::{Either, select};
use rand::Rng;
use serde::Serialize;
use worker::kv::KvStore;
use worker::{Cache, Context, Delay, console_error, console_log, console_warn};

use crate::api::{DataResponseEnvelope, Freshness, PostBackupKind};
use crate::cache::{EtagJson, put_cdn_cache};
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

/// Per-`(environment, cache key)` bookkeeping for background refreshes of the persistent cache,
/// scoped to this isolate. It does two jobs:
///
/// 1. **Stampede protection.** If a refresh is already in flight for a key, we do not spawn
///    another. Otherwise every request arriving in the window after the edge cache expires would
///    hammer the upstream NocoDB instance.
///
/// 2. **Backoff.** When a refresh fails, we record a cooldown before another may be attempted,
///    growing exponentially with consecutive failures. Without this, a fast-failing NocoDB (say,
///    500s returned in 50ms) gets re-attempted at request arrival rate, and that load is precisely
///    what stops it from recovering.
///
/// This only works **within this isolate**. Under heavy load, Cloudflare may spin up multiple
/// isolates to handle requests. That may mean a few concurrent refreshes for the same key, or a
/// freshly-spawned isolate spending one attempt before it learns that upstream is unhealthy, but
/// for our use-case that's on the order of "a few" and not "a few hundred", which is acceptable.
///
/// We actually *do not* want global state across all isolates across all datacenters, because the
/// CDN cache (what we call the "edge cache" below) is scoped per-datacenter, so each datacenter
/// needs to refresh independently anyways.
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
const REFRESH_TIMEOUT: Duration = Duration::from_secs(15);

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
fn now_ms() -> i64 {
    chrono::Utc::now().timestamp_millis()
}

// How long to wait before attempting another refresh, after `consecutive_failures` failures in a
// row. Doubles per failure up to `MAX_BACKOFF`, with equal jitter (half the delay fixed, half
// random) so isolates that failed at the same moment don't all retry in lockstep.
fn backoff_delay(consecutive_failures: u32) -> Duration {
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

// Run an upstream request, giving up after `REFRESH_TIMEOUT`. Dropping the losing future cancels
// the in-flight `fetch`, and `Delay` cancels its timer when dropped, so neither leaks.
async fn with_refresh_timeout<F, T>(key: &str, request: F) -> Option<T>
where
    F: Future<Output = Option<T>>,
{
    let request = pin!(request);
    let timeout = pin!(Delay::from(REFRESH_TIMEOUT));

    match select(request, timeout).await {
        Either::Left((value, _)) => value,
        Either::Right(((), _)) => {
            console_warn!(
                "Refresh for {} timed out after {}s.",
                key,
                REFRESH_TIMEOUT.as_secs(),
            );

            None
        }
    }
}

pub struct Store {
    noco_client: NocoClient,
    neon_client: NeonClient,
    kv: KvStore,
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

// This macro generates a method on the `Store` for fetching data from NocoDB with caching.
//
// We're using the Cloudflare cache API with a short TTL (configurable per-environment, but likely
// on the order of seconds) to reduce the load on the upstream NocoDB instance. We'll call this the
// "edge cache". Using this edge cache means incoming requests will only be routed upstream at most
// once every *n* milliseconds per data center per cache key. This is important, because the NocoDB
// instance is slower and much more expensive to scale than this worker.
//
// Once the edge cache expires, we need to fetch fresh data from NocoDB. However, this runs the
// risk of a cache stampede, where many requests arriving all at once hammer the upstream NocoDB
// instance. To remedy this, we have a second cache stored in KV that never expires. We'll call
// this the "persistent cache". Requests that miss the edge cache will hit the persistent cache
// instead, which will then kick off a background task (that does not block the request from
// returning) to update both caches with fresh data from NocoDB. We also implement a locking
// mechanism to ensure that only one request at a time can trigger this background refresh,
// otherwise we would have the same cache stampede problem. See `REFRESH_STATE`.
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
        type_name: $type_name:ty,
        get_api_fn: $get_api_fn:path,
        get_cached_fn: $get_cached_fn:path,
        put_cached_fn: $put_cached_fn:path,
        cache_key: $cache_key:expr,
    } => {
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

            // Required because `Context::wait_until` requires a static future.
            let kv_for_upstream = self.kv.clone();
            let env_name_for_upstream = self.env_name.clone();
            let noco_client_for_upstream = self.noco_client.clone();
            let base_id_for_upstream = self.base_id.clone();

            // A request to get the most recent data from NocoDB.
            let upstream_request = async move {
                match Self::get_table_ids(&kv_for_upstream, &env_name_for_upstream, &noco_client_for_upstream, &base_id_for_upstream).await {
                    Ok(table_ids) => {
                        match $get_api_fn(&noco_client_for_upstream, &table_ids)
                            .await
                        {
                            Ok(value) => Some(value),
                            Err(e) => {
                                console_warn!("Failed getting {} from NocoDB: {}", $cache_key, e);
                                None
                            }
                        }
                    },
                    Err(e) => {
                        console_warn!("Failed getting table IDs from NocoDB: {}", e);
                        None
                    }
                }
            };

            let kv_for_cache = self.kv.clone();
            let env_name_for_cache = self.env_name.clone();
            let env_name_for_cdn = self.env_name.clone();
            let cache_ttl = self.cache_ttl();

            // Refresh both the edge cache and the persistent cache.
            let put_cache = async move |value: $type_name, body: T| {
                console_log!("Caching latest {} from NocoDB.", $cache_key);

                if let Err(e) = $put_cached_fn(&kv_for_cache, &env_name_for_cache, &value).await {
                    console_warn!("Failed putting {} in KV cache: {}", $cache_key, e);
                }

                // Responses that hit the edge cache are by definition fresh; this entry is only
                // written after a successful refresh. Marking it otherwise would catch the client
                // in an infinite retry loop.
                let response_for_edge_cache_result = worker::Response::try_from(
                    EtagJson(DataResponseEnvelope {
                        freshness: Freshness::Fresh,
                        value: body,
                    })
                    .into_response()
                );

                let response_for_edge_cache = match response_for_edge_cache_result {
                    Ok(response) => response,
                    Err(_) => return,
                };

                put_cdn_cache(
                    &Cache::default(),
                    env_name_for_cdn,
                    cache_ttl,
                    uri,
                    response_for_edge_cache,
                ).await;
            };

            match cached_value {
                Some(cached_value) => {
                    let to_body_for_cache = to_body.clone();
                    let body = to_body(cached_value);

                    let refresh_key = format!("{}:{}", self.env_name, $cache_key);
                    let (action, freshness) = try_begin_refresh(&refresh_key);

                    match action {
                        RefreshAction::Spawn => {
                            self.ctx.wait_until(async move {
                                let latest = with_refresh_timeout(
                                    &refresh_key,
                                    upstream_request,
                                ).await;

                                let succeeded = latest.is_some();

                                if let Some(latest_value) = latest {
                                    let latest_body = to_body_for_cache(latest_value.clone());
                                    put_cache(latest_value, latest_body).await;
                                }

                                finish_refresh(&refresh_key, succeeded);
                            });
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

                    Ok(EtagJson(DataResponseEnvelope {
                        freshness,
                        value: body,
                    })
                    .into_response())
                },
                None => {
                    // The persistent cache is empty, which should only be the case for new
                    // environments or after the cache is manually cleared. We need to block and
                    // wait for the upstream request, because we have nothing else to serve.
                    let refresh_key = format!("{}:{}", self.env_name, $cache_key);

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

                    // Deliberately no in-flight guard and no timeout on this path. A legitimate
                    // cold start — a new environment, or one whose cache was just cleared — has to
                    // be allowed to block and fetch, however slow NocoDB is to wake up.
                    let latest = upstream_request.await;

                    finish_refresh(&refresh_key, latest.is_some());

                    if let Some(latest_value) = latest {
                        let body = to_body(latest_value.clone());
                        let body_for_cache = body.clone();

                        self.ctx.wait_until(async move {
                            put_cache(latest_value, body_for_cache).await;
                        });

                        // This data came straight from NocoDB, so it's as current as it gets. The
                        // edge cache is populated in the background above, so there's nothing for
                        // the client to check back for.
                        Ok(EtagJson(DataResponseEnvelope {
                            freshness: Freshness::Fresh,
                            value: body,
                        })
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
        self.env_config
            .cache_ttl
            .map(Duration::from_millis)
            .unwrap_or(config::noco_default_cdn_cache_ttl())
    }

    async fn get_table_ids(
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
        type_name: Vec<noco::Event>,
        get_api_fn: noco::get_events,
        get_cached_fn: kv::get_cached_events,
        put_cached_fn: kv::put_cached_events,
        cache_key: "events",
    }

    get_data! {
        fn_name: get_people,
        type_name: Vec<noco::Person>,
        get_api_fn: noco::get_people,
        get_cached_fn: kv::get_cached_people,
        put_cached_fn: kv::put_cached_people,
        cache_key: "people",
    }

    get_data! {
        fn_name: get_info,
        type_name: noco::Info,
        get_api_fn: noco::get_info,
        get_cached_fn: kv::get_cached_info,
        put_cached_fn: kv::put_cached_info,
        cache_key: "info",
    }

    get_data! {
        fn_name: get_pages,
        type_name: Vec<noco::Page>,
        get_api_fn: noco::get_pages,
        get_cached_fn: kv::get_cached_pages,
        put_cached_fn: kv::put_cached_pages,
        cache_key: "pages",
    }

    get_data! {
        fn_name: get_announcements,
        type_name: Vec<noco::Announcement>,
        get_api_fn: noco::get_announcements,
        get_cached_fn: kv::get_cached_announcements,
        put_cached_fn: kv::put_cached_announcements,
        cache_key: "announcements",
    }

    get_data! {
        fn_name: get_files,
        type_name: Vec<noco::File>,
        get_api_fn: noco::get_files,
        get_cached_fn: kv::get_cached_files,
        put_cached_fn: kv::put_cached_files,
        cache_key: "files",
    }

    // Refresh the cache specifically with the latest announcements from NocoDB. This is necessary
    // because we send out push notifications for announcements.
    #[worker::send]
    pub async fn refresh_announcements_cache(&self) -> Result<(), Error> {
        let table_ids =
            Self::get_table_ids(&self.kv, &self.env_name, &self.noco_client, &self.base_id).await?;

        let announcements = noco::get_announcements(&self.noco_client, &table_ids)
            .await
            .map_err(Error::Internal)?;

        // Refresh the persistent cache.
        kv::put_cached_announcements(&self.kv, &self.env_name, &announcements)
            .await
            .map_err(Error::Internal)?;

        // Purge the edge cache for this environment so incoming requests hit the persistent cache.
        cf::Client::new()
            .purge_cache(
                &config::cloudflare_zone_id(),
                &cf::CacheTag::for_env(&self.env_name),
            )
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
