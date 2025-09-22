use crate::api::PostBackupKind;
use crate::env::{EnvId, EnvName};
use crate::error::Error;
use crate::neon::BackupSnapshot;
use crate::noco::{self, BaseId, ExistingMigrationState, MigrationState, TableIds};
use crate::router::AppState;
use crate::{config, kv, url};
use crate::{
    neon::Client as NeonClient,
    noco::Client as NocoClient,
    sql::{Client as DbClient, ConnectionConfig as DbConnectionConfig},
};
use futures::channel::oneshot;
use futures::future::{self, Either, FutureExt};
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use std::collections::hash_map::Entry as MapEntry;
use std::fmt;
use std::pin::pin;
use std::sync::{Arc, LazyLock, Mutex};
use std::time::Duration;
use worker::kv::KvStore;
use worker::{Context, Date, Delay, console_error, console_log, console_warn};

#[derive(Debug)]
pub struct DataResponseEnvelope<T> {
    pub retry_after: Option<Duration>,
    pub value: T,
}

pub struct Store {
    noco_client: NocoClient,
    neon_client: NeonClient,
    db_client: DbClient,
    kv: KvStore,
    ctx: Arc<Context>,
    env_name: EnvName,
    base_id: BaseId,
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

// If NocoDB is unavailable, we tell the client to retry after this delay.
const NOCO_UNAVAILABLE_RETRY_DELAY: Duration = Duration::from_secs(1);

// If the NocoDB healtcheck takes more than this long to return, we consider it to be unhealthy.
const NOCO_HEALTHCHECK_TIMEOUT: Duration = Duration::from_secs(1);

// A request will only wait this long for an in-flight request to return. Requests waiting for an
// in-flight request should always be notified. This serves to guard against bugs.
const INFLIGHT_LOCK_TIMEOUT: Duration = Duration::from_secs(30);

// An instant in time for the purpose of caching. This wraps a number of milliseconds since the
// Unix epoch. This is backed by the JS `Date` API, because we don't seem to have access to a
// proper monotonic clock in this environment.
#[derive(Debug, Clone, Copy)]
struct CacheInstant(u64);

impl CacheInstant {
    pub fn now() -> Self {
        Self(Date::now().as_millis())
    }

    pub fn elapsed(&self) -> Duration {
        let now = Date::now();
        Duration::from_millis(now.as_millis().saturating_sub(self.0))
    }
}

// In the in-memory cache, we differentiate between values which have never been cached and values
// which were cached at one point but have since expired. This is necessary to facilitate an
// optimization where new isolates have their in-memory cache warmed from the KV cache.
// Previously-cached-but-expired values should hit the upstream NocoDB instance; what we're trying
// to avoid is a burst of traffic spinning up multiple new isolates which all hit the upstream
// NocoDB instance simultaneously.
//
// Rather than just letting the expired value sit in the cache forever, we have a separate
// `Expired` variant to minimize the cache's memory footprint.
#[derive(Debug)]
enum CacheEntry {
    Fresh {
        inserted_at: CacheInstant,
        serialized_value: String,
    },
    Expired,
    InFlight(Vec<oneshot::Sender<String>>),
}

static MEMORY_CACHE: LazyLock<Mutex<HashMap<String, CacheEntry>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

#[derive(Debug)]
enum CachedValue<T> {
    Fresh(T),
    Expired,
    Missing,
}

async fn get_memory_cache<T: DeserializeOwned>(key: &str, ttl: Duration) -> CachedValue<T> {
    #[derive(Debug)]
    enum SerializedValue {
        Ready(String),
        InFlight(oneshot::Receiver<String>),
    }

    let cached_value = {
        // We need to not be holding the lock over the await boundary while waiting for the in-flight
        // request to complete. Even in the case where the value was fresh, we don't need to hold the
        // lock while deserializing.
        let mut cache = MEMORY_CACHE.lock().unwrap();

        match cache.entry(key.to_string()) {
            MapEntry::Occupied(mut map_entry) => match map_entry.get_mut() {
                CacheEntry::Fresh {
                    inserted_at,
                    serialized_value,
                } if inserted_at.elapsed() < ttl => {
                    CachedValue::Fresh(SerializedValue::Ready(serialized_value.clone()))
                }
                cache_entry @ CacheEntry::Fresh { .. } => {
                    *cache_entry = CacheEntry::Expired;
                    CachedValue::Expired
                }
                CacheEntry::Expired => CachedValue::Expired,
                CacheEntry::InFlight(senders) => {
                    let (sender, receiver) = oneshot::channel();
                    senders.push(sender);

                    CachedValue::Fresh(SerializedValue::InFlight(receiver))
                }
            },
            MapEntry::Vacant(_) => CachedValue::Missing,
        }
    };

    match cached_value {
        CachedValue::Fresh(serialized_value) => {
            let serialized_string = match serialized_value {
                SerializedValue::Ready(s) => s,
                SerializedValue::InFlight(receiver) => match receiver.await {
                    Ok(s) => s,
                    Err(e) => {
                        console_warn!(
                            "Failed receiving in-flight cached value for key `{}`: {}",
                            key,
                            e
                        );
                        return CachedValue::Missing;
                    }
                },
            };

            match serde_json::from_str(&serialized_string) {
                Ok(value) => CachedValue::Fresh(value),
                Err(e) => {
                    console_warn!("Failed deserializing cached value for key `{}`: {}", key, e);

                    // As a defensive measure, if we fail to deserialize the cached value, we
                    // should evict it from the cache to avoid repeated errors on subsequent
                    // requests.
                    let mut cache = MEMORY_CACHE.lock().unwrap();
                    cache.remove(key);

                    CachedValue::Missing
                }
            }
        }
        CachedValue::Expired => CachedValue::Expired,
        CachedValue::Missing => CachedValue::Missing,
    }
}

#[derive(Debug, Clone, Copy)]
enum NotifyAction<T> {
    // Refresh the in-memory cache and notify all waiting requests.
    Refresh(T),

    // Expire the in-memory cache and notify all waiting requests.
    Expire(T),

    // Refresh the in-memory cache and do not notify any waiting requests.
    OnlyStore(T),

    // Expire the in-memory cache and close all waiting requests.
    Close,
}

async fn put_memory_cache<T: Serialize>(key: &str, action: NotifyAction<T>) -> anyhow::Result<()> {
    // We serialize first to minimize how long we're holding the lock.
    let serialized_value_action = match action {
        NotifyAction::Refresh(value) => NotifyAction::Refresh(serde_json::to_string(&value)?),
        NotifyAction::Expire(value) => NotifyAction::Expire(serde_json::to_string(&value)?),
        NotifyAction::OnlyStore(value) => NotifyAction::OnlyStore(serde_json::to_string(&value)?),
        NotifyAction::Close => NotifyAction::Close,
    };

    let waiters = {
        let mut cache = MEMORY_CACHE.lock().unwrap();

        match cache.entry(key.to_string()) {
            MapEntry::Occupied(mut map_entry) => {
                let waiters = if let CacheEntry::InFlight(senders) = map_entry.get_mut() {
                    if let NotifyAction::OnlyStore(_) = serialized_value_action {
                        Vec::new()
                    } else {
                        std::mem::take(senders)
                    }
                } else {
                    Vec::new()
                };

                match &serialized_value_action {
                    NotifyAction::Expire(_) | NotifyAction::Close => {
                        *map_entry.get_mut() = CacheEntry::Expired;
                    }
                    NotifyAction::Refresh(serialized_value)
                    | NotifyAction::OnlyStore(serialized_value) => {
                        *map_entry.get_mut() = CacheEntry::Fresh {
                            inserted_at: CacheInstant::now(),
                            serialized_value: serialized_value.clone(),
                        };
                    }
                };

                waiters
            }
            MapEntry::Vacant(_) => return Ok(()),
        }
    };

    if let NotifyAction::Refresh(serialized_value) | NotifyAction::Expire(serialized_value) =
        serialized_value_action
    {
        for waiter in waiters {
            if waiter.send(serialized_value.clone()).is_err() {
                console_warn!("Failed notifying in-flight waiter for key `{}`", key);
            }
        }
    }

    Ok(())
}

// This macro generates a method on the `Store` for fetching data from NocoDB with caching.
//
// TLDR: We're using aggressive caching to create an eventually consistent system that maximizes
// availability while allowing the NocoDB instance to shut itself off when not in use.
//
// An in-memory cache with a short TTL (configurable per-environment, but likely on the order of
// seconds) is used to reduce the load on the NocoDB instance. This means that, **for requests
// handled by this isolate**, the NocoDB instance will only be hit at most once every *n*
// milliseconds per cache key, and data returned to the client will only be stale by at most *n*
// milliseconds. This is important, because the NocoDB instance is slower and much more expensive
// to scale than this worker.
//
// However, this in-memory cache is **per-isolate**, meaning that if there's a lot of traffic or
// geographically distributed traffic, multiple isolates may be spun up, each with their own
// in-memory cache. This means that the load on the NocoDB instance *will* scale with the number of
// users, just not linearly.
//
// We need to handle the case where the NocoDB instance is temporarily unavailable. This may happen
// fairly often, because if the Fly Machine shuts itself off (which it is configured to do, to save
// money), it takes about 10 seconds to start back up, and during that time it will not respond to
// requests. To handle this, we also keep a persistent cache in KV, which is shared between
// isolates and used as a fallback for when the NocoDB instance is unavailable.
//
// This persistent cache serves a second purpose as well: we use it to warm the in-memory cache of
// new isolates so that a burst of traffic which spins up multiple new isolates doesn't cause a
// spike in load on the NocoDB instance.
//
// Whenever the worker responds to a request with expired data from the persistent cache, it
// includes a directive for the client to retry the request after a short delay, by which point the
// NocoDB instance is hopefully online. This saves the user from a 10+ second loading spinner while
// the NocoDB instance starts up.
macro_rules! get_data {
    {
        fn_name: $fn_name:ident,
        type_name: $type_name:ty,
        get_api_fn: $get_api_fn:path,
        get_cached_fn: $get_cached_fn:path,
        put_cached_fn: $put_cached_fn:path,
        cache_key_fn: $cache_key_fn:path,
        err_msg_key: $err_msg_key:expr,
    } => {
        #[worker::send]
        pub async fn $fn_name(&self) -> Result<DataResponseEnvelope<$type_name>, Error> {
            let cache_key = $cache_key_fn(&self.env_name);
            let env_config = kv::get_env_config(&self.kv, &self.env_name).await.map_err(Error::Internal)?;
            let ttl = env_config
                .cache_ttl
                .map(Duration::from_millis)
                .unwrap_or(config::noco_default_memory_cache_ttl());

            // As a defensive measure against bugs, we set a cap on how long we will wait for an
            // in-flight request to return.
            let memory_cache_request = pin!(get_memory_cache(&cache_key, ttl));
            let memory_cache_timeout = pin!(Delay::from(INFLIGHT_LOCK_TIMEOUT));

            let memory_cache_response = future::select(
                memory_cache_request,
                memory_cache_timeout,
            ).map(|either| match either {
                Either::Left((response, _)) => Ok(response),
                Either::Right(_) => {
                    console_warn!("Timed out waiting for in-flight upstream request to return after {} seconds. This is a bug.", INFLIGHT_LOCK_TIMEOUT.as_secs());
                    Err(Error::Internal(anyhow::anyhow!("Timed out waiting for in-flight upstream request to return.")))
                }
            });

            // Check this isolate's in-memory cache first.
            match memory_cache_response.await? {
                CachedValue::Fresh(value) => {
                    console_log!("Returning cached {}; skipping NocoDB.", $err_msg_key);
                    return Ok(DataResponseEnvelope { value, retry_after: None });
                },
                CachedValue::Missing => {
                    // This value is not in the in-memory cache at all, which probably
                    // means this is a new isolate. In that case, we should fetch a possibly-stale
                    // value from the KV cache to warm this isolate's in-memory cache.
                    console_log!("No cached {} in memory; fetching from KV cache.", $err_msg_key);

                    match $get_cached_fn(&self.kv, &self.env_name).await {
                        Ok(Some(value)) if !value.expired => {
                            console_log!("Warming in-memory cache for {} from KV cache.", $err_msg_key);

                            if let Err(e) = put_memory_cache(&cache_key, NotifyAction::OnlyStore(&value.value)).await {
                                console_warn!("Failed putting {} in memory cache: {}", $err_msg_key, e);
                            }

                            return Ok(DataResponseEnvelope { value: value.value, retry_after: None });
                        }
                        Ok(_) => {
                            console_log!("No fresh {} found in KV cache.", $err_msg_key);
                        }
                        Err(e) => {
                            console_warn!("Failed getting {} from KV cache: {}", $err_msg_key, e);
                        }
                    }
                },
                CachedValue::Expired => {
                    console_log!("Cached {} expired; fetching from NocoDB.", $err_msg_key);
                },
            }

            // Register a lock to ensure that only one request to NocoDB is in-flight at a time.
            // Otherwise, a burst of traffic could trigger many requests to NocoDB. Requests that
            // come in between this point and when the upstream request to NocoDB completes will
            // wait to be notified via a oneshot channel when that upstream request completes.
            {
                let mut cache = MEMORY_CACHE.lock().unwrap();

                // Avoid a race condition by only inserting if the entry is still vacant.
                cache
                    .entry(cache_key.to_string())
                    .or_insert_with(|| CacheEntry::InFlight(Vec::new()));
            }

            // A request to get the most recent data from NocoDB.
            let value_request = async {
                let table_ids = match self.get_table_ids().await {
                    Ok(table_ids) => table_ids,
                    Err(e) => {
                        console_warn!("Failed getting table IDs from NocoDB: {}", e);
                        return Err(e);
                    }
                };

                match $get_api_fn(&self.noco_client, &table_ids)
                    .await
                    .map_err(Error::Internal)
                {
                    Ok(value) => Ok(value),
                    Err(e) => {
                        console_warn!("Failed getting {} from NocoDB: {}", $err_msg_key, e);
                        Err(e)
                    }
                }
            };

            // A request to NocoDB's healthcheck endpoint.
            let healthcheck_request = pin!(noco::check_health(&self.noco_client));

            let healthcheck_timeout = pin!(Delay::from(NOCO_HEALTHCHECK_TIMEOUT));

            // Perform a healthcheck on the NocoDB server with a timeout.
            //
            // The timeout is important, because API requests to NocoDB tend to hang while the Fly
            // Machine is starting up.
            let healthcheck = future::select(
                healthcheck_request,
                healthcheck_timeout,
            ).map(|either| match either {
                Either::Left((is_healthy, _)) => is_healthy,
                Either::Right(_) => {
                    console_warn!("NocoDB healthcheck timed out after {} seconds.", NOCO_HEALTHCHECK_TIMEOUT.as_secs());
                    false
                }
            });

            // This is important! We're both fetching the latest data from NocoDB *and* checking
            // whether it's healthy, in parallel.
            //
            // - If the healthcheck times out, indicating that NocoDB is unhealthy, then we will
            // not await the API request, and will instead go straight to the cache.
            // - If the healthcheck returns first and indicates that NocoDB is unhealthy, then we
            // will not await the API request, and will instead go straight to the cache.
            // - If the healthcheck returns first and indicates that NocoDB is healthy, then will
            // await the API request and return it.
            // - If the API request returns first (unlikely), we will not await the healthcheck.
            let maybe_value_if_healthy = match future::select(pin!(value_request), pin!(healthcheck)).await {
                Either::Left((value_result, _)) => Some(value_result?),
                Either::Right((is_healthy, value_future)) => if is_healthy {
                    Some(value_future.await?)
                } else {
                    None
                }
            };

            match maybe_value_if_healthy {
                Some(value) => {
                    console_log!("Caching latest {} from NocoDB.", $err_msg_key);

                    // Notify other requests that came in at the same time as this one that we have
                    // received fresh data from NocoDB and send it to them. Also refresh the
                    // in-memory cache.
                    if let Err(e) = put_memory_cache(&cache_key, NotifyAction::Refresh(&value)).await {
                        console_warn!("Failed putting {} in memory cache: {}", $err_msg_key, e);
                    }

                    let kv_for_cache = self.kv.clone();
                    let value_for_cache = value.clone();
                    let env_name_for_cache = self.env_name.clone();

                    // Update the KV cache used to warm new isolates. This doesn't need to block
                    // the current request.
                    self.ctx.wait_until(async move {
                        if let Err(e) = $put_cached_fn(&kv_for_cache, &env_name_for_cache, &value_for_cache).await {
                            console_warn!("Failed putting {} in KV cache: {}", $err_msg_key, e);
                        }
                    });

                    Ok(DataResponseEnvelope { value, retry_after: None })
                },
                None => {
                    console_log!("NocoDB is unavailable, fetching stale data if available.");

                    match $get_cached_fn(&self.kv, &self.env_name).await {
                        Ok(Some(value)) => {
                            // Return the cached data whether or not it has expired, because we
                            // have nothing better to send the client right now.
                            console_log!("Returning stale {} from cache.", $err_msg_key);

                            if let Err(e) = put_memory_cache(&cache_key, NotifyAction::Expire(&value.value)).await {
                                console_warn!("Failed putting {} in memory cache: {}", $err_msg_key, e);
                            }

                            Ok(DataResponseEnvelope { value: value.value, retry_after: Some(NOCO_UNAVAILABLE_RETRY_DELAY) })
                        }
                        Ok(None) => {
                            console_warn!("No stale cached {} found.", $err_msg_key);

                            if let Err(e) = put_memory_cache::<()>(&cache_key, NotifyAction::Close).await {
                                console_warn!("Failed closing in-flight waiters for {}: {}", $err_msg_key, e);
                            }

                            Err(Error::NocoUnavailable)
                        }
                        Err(e) => {
                            console_warn!("Failed getting {} from cache: {}", $err_msg_key, e);

                            if let Err(e) = put_memory_cache::<()>(&cache_key, NotifyAction::Close).await {
                                console_warn!("Failed closing in-flight waiters for {}: {}", $err_msg_key, e);
                            }

                            Err(Error::Internal(e))
                        }
                    }
                }
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

        let db_client = DbClient::connect(
            &Option::<DbConnectionConfig>::from(env_config).ok_or(Error::MissingEnvConfig)?,
        )
        .await
        .map_err(Error::Internal)?;

        let base_id = db_client
            .get_base()
            .await
            .map_err(Error::Internal)?
            .ok_or(Error::NoBaseId)?;

        let dash_origin = url::dash_origin(&env_name).map_err(Error::Internal)?;

        let noco_client = NocoClient::new(dash_origin.clone(), api_token);
        let neon_client = NeonClient::new();

        Ok(Self {
            noco_client,
            neon_client,
            db_client,
            kv,
            ctx,
            env_name,
            base_id,
        })
    }

    pub async fn from_env_id(state: &AppState, env_id: &EnvId) -> Result<Self, Error> {
        let env_name = kv::get_id_env(&state.kv, env_id)
            .await
            .map_err(Error::Internal)?
            .ok_or(Error::NoEnvId)?;

        Self::from_env_name(state, env_name).await
    }

    async fn get_table_ids(&self) -> Result<TableIds, Error> {
        Ok(
            match kv::get_tables(&self.kv, &self.env_name)
                .await
                .and_then(TableIds::try_from)
            {
                Ok(table_ids) => table_ids,
                Err(e) => {
                    console_log!("Failed to get table IDs from KV: {}", e);

                    let table_ids = noco::list_tables(&self.noco_client, &self.base_id)
                        .await
                        .map_err(Error::Internal)?;

                    kv::put_tables(&self.kv, &self.env_name, &table_ids)
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
        cache_key_fn: kv::events_cache_key,
        err_msg_key: "events",
    }

    get_data! {
        fn_name: get_info,
        type_name: noco::Info,
        get_api_fn: noco::get_info,
        get_cached_fn: kv::get_cached_info,
        put_cached_fn: kv::put_cached_info,
        cache_key_fn: kv::info_cache_key,
        err_msg_key: "info",
    }

    get_data! {
        fn_name: get_pages,
        type_name: Vec<noco::Page>,
        get_api_fn: noco::get_pages,
        get_cached_fn: kv::get_cached_pages,
        put_cached_fn: kv::put_cached_pages,
        cache_key_fn: kv::pages_cache_key,
        err_msg_key: "pages",
    }

    get_data! {
        fn_name: get_announcements,
        type_name: Vec<noco::Announcement>,
        get_api_fn: noco::get_announcements,
        get_cached_fn: kv::get_cached_announcements,
        put_cached_fn: kv::put_cached_announcements,
        cache_key_fn: kv::announcements_cache_key,
        err_msg_key: "announcements",
    }

    get_data! {
        fn_name: get_about,
        type_name: noco::About,
        get_api_fn: noco::get_about,
        get_cached_fn: kv::get_cached_about,
        put_cached_fn: kv::put_cached_about,
        cache_key_fn: kv::about_cache_key,
        err_msg_key: "about",
    }

    // We add an additional layer of caching here with a longer TTL, because this endpoint can
    // block the entire app from loading.
    pub async fn get_summary(&self) -> Result<noco::Summary, Error> {
        match kv::get_cached_summary(&self.kv, &self.env_name).await {
            Ok(Some(value)) => {
                console_log!(
                    "Returning summary from separate cache with TTL {}s; skipping NocoDB.",
                    config::noco_summary_cache_ttl().as_secs()
                );
                return Ok(value);
            }
            Ok(None) => {
                console_log!("No cached summary found, fetching from NocoDB.");
            }
            Err(e) => {
                console_warn!("Failed to get summary from cache: {}", e);
            }
        }

        let DataResponseEnvelope { value: about, .. } = self.get_about().await?;

        let summary = noco::Summary {
            name: about.name.clone(),
            description: about.description,
        };

        kv::put_cached_summary(&self.kv, &self.env_name, &summary)
            .await
            .map_err(Error::Internal)?;

        Ok(summary)
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
        let old_version = self
            .db_client
            .get_current_migration()
            .await
            .map_err(Error::Internal)?;

        let migration_state = MigrationState::existing(old_version, self.base_id.clone());

        let migrator = noco::Migrator::new(&self.noco_client, &self.neon_client, &self.db_client);

        let ExistingMigrationState {
            version: new_version,
            ..
        } = migrator
            .migrate(&self.env_name, migration_state)
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

        self.neon_client
            .with_rollback(&self.env_name, async || {
                let result = {
                    noco::delete_base(&self.noco_client, &self.base_id).await?;
                    self.db_client.delete_base(&self.base_id).await?;
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
