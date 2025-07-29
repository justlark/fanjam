use crate::api::{PostBackupKind, PostRestoreBackupKind};
use crate::env::{EnvId, EnvName};
use crate::error::Error;
use crate::noco::{
    self, BaseId, ExistingMigrationState, MigrationState, NOCO_PRE_BASE_DELETION_BRANCH_NAME,
    NOCO_PRE_DEPLOYMENT_BRANCH_NAME, NOCO_PRE_MANUAL_RESTORE_BRANCH_NAME, TableIds,
    noco_migration_branch_name,
};
use crate::{kv, neon, url};
use crate::{neon::Client as NeonClient, noco::Client as NocoClient};
use futures::future::{self, Either, FutureExt};
use std::fmt;
use std::io;
use std::pin::pin;
use std::time::Duration;
use wasm_timer::TryFutureExt;
use worker::kv::KvStore;
use worker::{console_log, console_warn};

#[derive(Debug)]
pub struct DataResponseEnvelope<T> {
    pub retry_after: Option<Duration>,
    pub value: T,
}

pub struct Store {
    noco_client: NocoClient,
    neon_client: NeonClient,
    kv: KvStore,
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

const NOCO_UNAVAILABLE_RETRY_DELAY: Duration = Duration::from_secs(1);
const NOCO_HEALTHCHECK_TIMEOUT: Duration = Duration::from_secs(1);

// This macro generates a method on the `Store` for fetching data from NocoDB with caching.
//
// TLDR: We're using aggressive caching to create an eventually consistent system that maximizes
// availability while allowing the NocoDB instance to shut itself off when not in use.
//
// There are two caches, which we'll call the "cache" and the "store". The cache has a short TTL
// and is used to reduce the load on the NocoDB instance, which is slower and more expensive than
// this worker. Data we fetch from NocoDB is put into the cache, where it expires after *n*
// seconds. Requests that come in from the client during that time will get the cached data. This
// means that NocoDB will only get hit at most every *n* seconds, and data returned to the client
// will only be stale by at most *n* seconds.
//
// Whenever we fetch data from NocoDB, we also put it into the store, where it does not expire. The
// purpose of the store is to handle the specific case where the NocoDB instance is temporarily
// unavailable, but the cache is empty. This may happen fairly often, because if the Fly Machine
// shuts itself off (which it is configured to do, to save money), it takes about 10 seconds to
// start back up, and during that time it will not respond to requests.
//
// Whenever the worker responds to a request with data from the store, it includes a directive for
// the client to retry the request after a short delay, by which point the NocoDB instance is
// hopefully online. This saves the user from a 10+ second loading spinner while the NocoDB
// instance starts up.
//
// We can't make any guarantees about how stale the data in the store is, but it's not particularly
// important, because that data will only be shown to the user for a few seconds until the client
// is able to fetch the latest data from NocoDB.
macro_rules! get_data {
    {
        fn_name: $fn_name:ident,
        type_name: $type_name:ty,
        get_api_fn: $get_api_fn:path,
        get_cached_fn: $get_cached_fn:path,
        put_cached_fn: $put_cached_fn:path,
        get_stored_fn: $get_stored_fn:path,
        put_stored_fn: $put_stored_fn:path,
        err_msg_key: $err_msg_key:expr,
    } => {
        pub async fn $fn_name(&self) -> Result<DataResponseEnvelope<$type_name>, Error> {
            match $get_cached_fn(&self.kv, &self.env_name).await {
                // When the value is fetched from this cache, we do not instruct the client to
                // retry the request. This data will always be reasonably fresh, and instructing
                // the client to retry the request would get it stuck in a refresh loop.
                Ok(Some(value)) => {
                    console_log!("Returning cached {}; skipping NocoDB.", $err_msg_key);
                    return Ok(DataResponseEnvelope { value, retry_after: None });
                },
                Ok(None) => {
                    console_log!("No cached {} found, fetching from NocoDB.", $err_msg_key);
                }
                Err(e) => {
                    console_warn!("Failed to get {} from cache: {}", $err_msg_key, e);
                }
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
            //
            // The timeout is important, because API requests to NocoDB tend to hang while the Fly
            // Machine is starting up.
            let healthcheck_request = pin!(noco::check_health(&self.noco_client).map(io::Result::Ok).timeout(
                NOCO_HEALTHCHECK_TIMEOUT,
            ));

            // This is important! We're both fetching the latest data from NocoDB *and* checking
            // whether it's healthy, in parallel.
            //
            // - If the healthcheck times out, indicating that NocoDB is unhealthy, then we will
            // not await the API request, and will instead go straight to the store.
            // - If the healthcheck returns first and indicates that NocoDB is unhealthy, then we
            // will not await the API request, and will instead go straight to the store.
            // - If the healthcheck returns first and indicates that NocoDB is healthy, then will
            // await the API request and return it.
            // - If the API request returns first (unlikely), we will not await the healthcheck.
            let maybe_value_if_healthy = match future::select(pin!(value_request), pin!(healthcheck_request)).await {
                Either::Left((value_result, _)) => Some(value_result?),
                Either::Right((is_healthy, value_future)) => if is_healthy.unwrap_or(false) {
                    Some(value_future.await?)
                } else {
                    None
                }
            };

            let envelope: DataResponseEnvelope<$type_name> = match maybe_value_if_healthy {
                Some(value) => DataResponseEnvelope { value, retry_after: None },
                None => {
                    console_log!("NocoDB is unavailable, fetching istale data if available.");

                    match $get_stored_fn(&self.kv, &self.env_name).await {
                        Ok(Some(value)) => {
                            console_log!("Returning stale {} from store.", $err_msg_key);
                            DataResponseEnvelope { value, retry_after: Some(NOCO_UNAVAILABLE_RETRY_DELAY) }
                        }
                        Ok(None) => {
                            console_warn!("No cached or stored {} found.", $err_msg_key);
                            return Err(Error::NocoUnavailable)
                        }
                        Err(e) => {
                            console_warn!("Failed getting {} from store: {}", $err_msg_key, e);
                            return Err(Error::Internal(e))
                        }
                    }
                }
            };

            if let Err(e) = $put_cached_fn(&self.kv, &self.env_name, &envelope.value).await {
                console_warn!("Failed putting {} in cache: {}", $err_msg_key, e);
            }

            if let Err(e) = $put_stored_fn(&self.kv, &self.env_name, &envelope.value).await {
                console_warn!("Failed putting {} in store: {}", $err_msg_key, e);
            }

            Ok(envelope)
        }
    }
}

impl Store {
    pub async fn from_env_name(kv: KvStore, env_name: EnvName) -> Result<Self, Error> {
        let api_token = kv::get_api_token(&kv, &env_name)
            .await
            .map_err(Error::Internal)?
            .ok_or(Error::NoApiToken)?;

        let base_id = kv::get_base_id(&kv, &env_name)
            .await
            .map_err(Error::Internal)?
            .ok_or(Error::NoBaseId)?;

        let dash_origin = url::dash_origin(&env_name).map_err(Error::Internal)?;

        let noco_client = noco::Client::new(dash_origin.clone(), api_token);

        let neon_client = neon::Client::new();

        Ok(Self {
            noco_client,
            neon_client,
            kv,
            env_name,
            base_id,
        })
    }

    pub async fn from_env_id(kv: KvStore, env_id: &EnvId) -> Result<Self, Error> {
        let env_name = kv::get_id_env(&kv, env_id)
            .await
            .map_err(Error::Internal)?
            .ok_or(Error::NoEnvId)?;

        Self::from_env_name(kv, env_name).await
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
        get_stored_fn: kv::get_stored_events,
        put_stored_fn: kv::put_stored_events,
        err_msg_key: "events",
    }

    get_data! {
        fn_name: get_info,
        type_name: noco::Info,
        get_api_fn: noco::get_info,
        get_cached_fn: kv::get_cached_info,
        put_cached_fn: kv::put_cached_info,
        get_stored_fn: kv::get_stored_info,
        put_stored_fn: kv::put_stored_info,
        err_msg_key: "info",
    }

    get_data! {
        fn_name: get_pages,
        type_name: Vec<noco::Page>,
        get_api_fn: noco::get_pages,
        get_cached_fn: kv::get_cached_pages,
        put_cached_fn: kv::put_cached_pages,
        get_stored_fn: kv::get_stored_pages,
        put_stored_fn: kv::put_stored_pages,
        err_msg_key: "pages",
    }

    pub async fn create_backup(&self, kind: PostBackupKind) -> Result<(), Error> {
        let dest_branch_name = match kind {
            PostBackupKind::Deployment => NOCO_PRE_DEPLOYMENT_BRANCH_NAME,
        };

        self.neon_client
            .create_backup(&self.env_name.clone().into(), &dest_branch_name)
            .await
            .map_err(Error::Internal)?;

        Ok(())
    }

    pub async fn restore_backup(
        &self,
        kind: PostRestoreBackupKind,
        version: Option<noco::Version>,
    ) -> Result<(), Error> {
        let source_branch_name = match kind {
            PostRestoreBackupKind::Deletion => NOCO_PRE_BASE_DELETION_BRANCH_NAME,
            PostRestoreBackupKind::Deployment => NOCO_PRE_DEPLOYMENT_BRANCH_NAME,
            PostRestoreBackupKind::Migration => match version {
                Some(version) => noco_migration_branch_name(&version),
                None => return Err(Error::MissingMigrationVersion),
            },
        };

        self.neon_client
            .restore_backup(
                &self.env_name.clone().into(),
                &source_branch_name,
                &NOCO_PRE_MANUAL_RESTORE_BRANCH_NAME,
            )
            .await
            .map_err(Error::Internal)
    }

    pub async fn migrate(&self) -> Result<MigrationChange, Error> {
        let old_version = kv::get_migration_version(&self.kv, &self.env_name)
            .await
            .map_err(Error::Internal)?
            .unwrap_or(noco::Version::INITIAL);

        let migration_state = MigrationState::existing(old_version, self.base_id.clone());

        let migrator = noco::Migrator::new(&self.noco_client, &self.neon_client, &self.kv);

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

    pub async fn delete_base(&self) -> Result<(), Error> {
        // Back up the database in case we delete the NocoDB base accidentally.
        self.neon_client
            .create_backup(
                &self.env_name.clone().into(),
                &NOCO_PRE_BASE_DELETION_BRANCH_NAME,
            )
            .await
            .map_err(Error::Internal)?;

        noco::delete_base(&self.noco_client, &self.base_id)
            .await
            .map_err(Error::Internal)?;

        kv::delete_base_id(&self.kv, &self.env_name)
            .await
            .map_err(Error::Internal)?;

        kv::delete_migration_version(&self.kv, &self.env_name)
            .await
            .map_err(Error::Internal)?;

        Ok(())
    }
}
