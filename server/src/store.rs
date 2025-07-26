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
use std::fmt;
use worker::console_log;
use worker::kv::KvStore;

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

    pub async fn get_info(&self) -> Result<noco::Info, Error> {
        let table_ids = self.get_table_ids().await?;

        noco::get_info(&self.noco_client, &table_ids)
            .await
            .map_err(Error::Internal)
    }

    pub async fn get_events(&self) -> Result<Vec<noco::Event>, Error> {
        let table_ids = self.get_table_ids().await?;

        noco::get_events(&self.noco_client, &table_ids)
            .await
            .map_err(Error::Internal)
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
