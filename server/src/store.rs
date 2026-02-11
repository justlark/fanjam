use std::collections::HashSet;
use std::fmt;
use std::sync::{Arc, Mutex, OnceLock};
use std::time::Duration;

use axum::{
    body::Body,
    http::{self, Uri},
    response::IntoResponse,
};
use serde::Serialize;
use worker::kv::KvStore;
use worker::{Cache, Context, console_error, console_log, console_warn};

use crate::api::{DataResponseEnvelope, PostBackupKind};
use crate::cache::{EtagJson, put_cdn_cache};
use crate::env::{Config, EnvId, EnvName};
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

/// Tracks which cache keys currently have a background refresh in flight within this isolate. This
/// is used to prevent cache stampedes; if a key is in this set, we do not spawn another background
/// task to refresh the cache from the upstream NocoDB instance. The background task removes the
/// key when it completes.
///
/// This locking mechanism only works **within this isolate**. Under heavy load, Cloudflare may
/// spin up multiple isolates to handle requests. That may result in multiple concurrent background
/// refreshes for the same key, but for our use-case it would likely be on the order of "a few" and
/// not "a few hundred", which is acceptable.
///
/// We actually *do not* want a global lock across all isolates across all datacenters, because the
/// CDN cache (what we call the "edge cache" below) is scoped per-datacenter, so we would need each
/// datacenter to have an independent lock anyways.
///
/// Workers isolates are inherently single-threaded, so the Mutex never actually contends; it's
/// only needed to satisfy Send/Sync bounds.
static INFLIGHT_REFRESHES: OnceLock<Mutex<HashSet<String>>> = OnceLock::new();

fn inflight_refreshes() -> &'static Mutex<HashSet<String>> {
    INFLIGHT_REFRESHES.get_or_init(|| Mutex::new(HashSet::new()))
}

pub struct Store {
    noco_client: NocoClient,
    neon_client: NeonClient,
    db_client: DbClient,
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
// otherwise we would have the same cache stampede problem.
//
// Whenever the worker responds to a request with expired data from the persistent cache, it
// includes a directive for the client to retry the request after a short delay, by which point the
// persistent cache in KV should have been updated with fresh data from NocoDB.
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
            F: FnOnce($type_name) -> T + 'static,
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

                // We consider responses that hit the edge cache to be fresh, so we set `stale` to
                // false. Otherwise the client would get caught in an infinite retry loop.
                let response_for_edge_cache_result = worker::Response::try_from(
                    EtagJson(DataResponseEnvelope {
                        stale: false,
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
                    let body = to_body(cached_value);

                    let refresh_key = format!("{}:{}", self.env_name, $cache_key);
                    let already_refreshing = {
                        let mut set = inflight_refreshes().lock().unwrap();
                        !set.insert(refresh_key.clone())
                    };

                    if !already_refreshing {
                        let body_for_cache = body.clone();

                        self.ctx.wait_until(async move {
                            if let Some(latest_value) = upstream_request.await {
                                put_cache(latest_value, body_for_cache).await;
                            }
                            inflight_refreshes().lock().unwrap().remove(&refresh_key);
                        });
                    } else {
                        console_log!(
                            "Skipping background refresh for {} (already in flight).",
                            $cache_key,
                        );
                    }

                    Ok(EtagJson(DataResponseEnvelope {
                        stale: true,
                        value: body,
                    })
                    .into_response())
                },
                None => {
                    // The persistent cache is empty, which should only be the case for new
                    // environments or after the cache is manually cleared. We need to block and
                    // wait for the upstream request.
                    if let Some(latest_value) = upstream_request.await {
                        let body = to_body(latest_value.clone());
                        let body_for_cache = body.clone();

                        self.ctx.wait_until(async move {
                            put_cache(latest_value, body_for_cache).await;
                        });

                        Ok(EtagJson(DataResponseEnvelope {
                            stale: true,
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
            env_config,
        })
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
