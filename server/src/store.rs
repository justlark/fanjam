use crate::api::{PostBackupKind, PostRestoreBackupKind};
use crate::env::{EnvId, EnvName};
use crate::error::Error;
use crate::http::StatusError;
use crate::noco::{
    self, BaseId, ExistingMigrationState, MigrationState, NOCO_PRE_BASE_DELETION_BRANCH_NAME,
    NOCO_PRE_DEPLOYMENT_BRANCH_NAME, NOCO_PRE_MANUAL_RESTORE_BRANCH_NAME, TableIds,
    noco_migration_branch_name,
};
use crate::{kv, neon, url};
use crate::{neon::Client as NeonClient, noco::Client as NocoClient};
use axum::http::StatusCode;
use std::fmt;
use worker::kv::KvStore;
use worker::{console_log, console_warn};

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

fn map_noco_api_error(err: anyhow::Error) -> Error {
    if let Some(status_err) = err.downcast_ref::<StatusError>() {
        if status_err.code() == StatusCode::SERVICE_UNAVAILABLE {
            return Error::NocoUnavailable;
        }
    }
    Error::Internal(err)
}

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
        pub async fn $fn_name(&self) -> Result<$type_name, Error> {
            match $get_cached_fn(&self.kv, &self.env_name).await {
                Ok(Some(events)) => return Ok(events),
                Ok(None) => {
                    console_log!("No cached {} found, fetching from NocoDB.", $err_msg_key);
                }
                Err(e) => {
                    console_warn!("Failed to get {} from cache: {}", $err_msg_key, e);
                }
            }

            let table_ids = match self.get_table_ids().await {
                Ok(table_ids) => table_ids,
                Err(e) => {
                    if matches!(e, Error::NocoUnavailable) {
                        return $get_stored_fn(&self.kv, &self.env_name)
                            .await
                            .map_err(Error::Internal)?
                            .ok_or(Error::NocoUnavailable);
                    }

                    return Err(e);
                }
            };

            let events = match $get_api_fn(&self.noco_client, &table_ids)
                .await
                .map_err(map_noco_api_error)
            {
                Ok(events) => events,
                Err(e) => {
                    console_warn!("Failed getting {}, from NocoDB: {}", $err_msg_key, e);

                    if matches!(e, Error::NocoUnavailable) {
                        console_log!("NocoDB is unavailable, returning stored data.");

                        return $get_stored_fn(&self.kv, &self.env_name)
                            .await
                            .map_err(Error::Internal)?
                            .ok_or(Error::NocoUnavailable);
                    }

                    return Err(e);
                }
            };

            if let Err(e) = $put_cached_fn(&self.kv, &self.env_name, &events).await {
                console_warn!("Failed putting {} in cache: {}", $err_msg_key, e);
            }

            if let Err(e) = $put_stored_fn(&self.kv, &self.env_name, &events).await {
                console_warn!("Failed putting {} in store: {}", $err_msg_key, e);
            }

            Ok(events)
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
                        .map_err(map_noco_api_error)?;

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
