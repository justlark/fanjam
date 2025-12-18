use crate::api::PostBackupKind;
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
use futures::future::{self, Either, FutureExt};
use std::fmt;
use std::pin::pin;
use std::sync::Arc;
use std::time::Duration;
use worker::kv::KvStore;
use worker::{Context, Delay, console_error, console_log, console_warn};

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

// If NocoDB is unavailable, we tell the client to retry after this delay.
const NOCO_UNAVAILABLE_RETRY_DELAY: Duration = Duration::from_secs(1);

// If the NocoDB healtcheck takes more than this long to return, we consider it to be unhealthy.
const NOCO_HEALTHCHECK_TIMEOUT: Duration = Duration::from_secs(1);

// This macro generates a method on the `Store` for fetching data from NocoDB with caching.
//
// We're using the Cloudflare cache API with a short TTL (configurable per-environment, but likely
// on the order of seconds) to reduce the load on the NocoDB instance. This means that the NocoDB
// instance will only be hit at most once every *n* milliseconds per data center per cache key, and
// data returned to the client will only be stale by at most *n* milliseconds. This is important,
// because the NocoDB instance is slower and much more expensive to scale than this worker.
//
// We need to handle the case where the NocoDB instance is temporarily unavailable. This may happen
// fairly often, because if the Fly Machine and/or Postgres Instance shut themselves off (which
// they are configured to do, to save money), they take some time to start back up, and during that
// time they will not respond to requests. To handle this, we also keep a persistent cache in KV,
// which is shared between isolates and used as a fallback for when the NocoDB instance is
// unavailable.
//
// Whenever the worker responds to a request with expired data from the persistent cache, it
// includes a directive for the client to retry the request after a short delay, by which point the
// NocoDB instance is hopefully online. This saves first-time users from a long loading spinner
// while the NocoDB instance starts up.
macro_rules! get_data {
    {
        fn_name: $fn_name:ident,
        type_name: $type_name:ty,
        get_api_fn: $get_api_fn:path,
        get_cached_fn: $get_cached_fn:path,
        put_cached_fn: $put_cached_fn:path,
        err_msg_key: $err_msg_key:expr,
    } => {
        #[worker::send]
        pub async fn $fn_name(&self) -> Result<DataResponseEnvelope<$type_name>, Error> {
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

                    let kv_for_cache = self.kv.clone();
                    let value_for_cache = value.clone();
                    let env_name_for_cache = self.env_name.clone();

                    // Update the persistent KV cache. This doesn't need to block the current
                    // request.
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

                            Ok(DataResponseEnvelope { value, retry_after: Some(NOCO_UNAVAILABLE_RETRY_DELAY) })
                        }
                        Ok(None) | Err(_) => {
                            console_warn!("No stale cached {} found.", $err_msg_key);

                            Err(Error::NocoUnavailable)
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

    pub fn cache_ttl(&self) -> Duration {
        self.env_config
            .cache_ttl
            .map(Duration::from_millis)
            .unwrap_or(config::noco_default_cdn_cache_ttl())
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
        err_msg_key: "events",
    }

    get_data! {
        fn_name: get_info,
        type_name: noco::Info,
        get_api_fn: noco::get_info,
        get_cached_fn: kv::get_cached_info,
        put_cached_fn: kv::put_cached_info,
        err_msg_key: "info",
    }

    get_data! {
        fn_name: get_pages,
        type_name: Vec<noco::Page>,
        get_api_fn: noco::get_pages,
        get_cached_fn: kv::get_cached_pages,
        put_cached_fn: kv::put_cached_pages,
        err_msg_key: "pages",
    }

    get_data! {
        fn_name: get_announcements,
        type_name: Vec<noco::Announcement>,
        get_api_fn: noco::get_announcements,
        get_cached_fn: kv::get_cached_announcements,
        put_cached_fn: kv::put_cached_announcements,
        err_msg_key: "announcements",
    }

    get_data! {
        fn_name: get_about,
        type_name: noco::About,
        get_api_fn: noco::get_about,
        get_cached_fn: kv::get_cached_about,
        put_cached_fn: kv::put_cached_about,
        err_msg_key: "about",
    }

    pub async fn get_summary(&self) -> Result<noco::Summary, Error> {
        let DataResponseEnvelope { value: about, .. } = self.get_about().await?;

        Ok(noco::Summary {
            name: about.name.clone(),
            description: about.description,
        })
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
