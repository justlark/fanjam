use std::fmt;

use worker::{console_error, kv::KvStore};

use crate::{env::EnvName, kv, neon::BranchName};

use super::{
    create_base,
    migrations::{self, BaseId, Client as NocoClient, Version},
};
use crate::neon::Client as NeonClient;

pub const NOCO_PRE_BASE_DELETION_BRANCH_NAME: BranchName =
    BranchName::new("noco-pre-base-deletion");
pub const NOCO_PRE_DEPLOYMENT_BRANCH_NAME: BranchName = BranchName::new("noco-pre-deployment");
pub const NOCO_PRE_MIGRATION_ROLLBACK_BRANCH_NAME: BranchName =
    BranchName::new("noco-pre-migration-rollback");
pub const NOCO_PRE_MANUAL_RESTORE_BRANCH_NAME: BranchName =
    BranchName::new("noco-pre-manual-restore");

fn noco_migration_branch_name(version: &Version) -> BranchName {
    format!("noco-migration-{}", version).into()
}

#[derive(Debug)]
pub struct ExistingMigrationState {
    pub version: Version,
    pub base_id: BaseId,
}

#[derive(Debug)]
pub struct NewMigrationState {
    pub title: String,
    pub initial_user_email: String,
}

#[derive(Debug)]
pub enum MigrationState {
    New(NewMigrationState),
    Existing(ExistingMigrationState),
}

impl MigrationState {
    pub fn new(title: String, initial_user_email: String) -> Self {
        MigrationState::New(NewMigrationState {
            title,
            initial_user_email,
        })
    }

    pub fn existing(version: Version, base_id: BaseId) -> Self {
        MigrationState::Existing(ExistingMigrationState { version, base_id })
    }
}

pub struct Migrator<'a> {
    noco_client: &'a NocoClient,
    neon_client: &'a NeonClient,
    kv: &'a KvStore,
}

impl fmt::Debug for Migrator<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Migrator")
            .field("noco_client", self.noco_client)
            .field("neon_client", self.neon_client)
            .finish_non_exhaustive()
    }
}

impl<'a> Migrator<'a> {
    pub fn new(noco_client: &'a NocoClient, neon_client: &'a NeonClient, kv: &'a KvStore) -> Self {
        Self {
            noco_client,
            neon_client,
            kv,
        }
    }

    #[worker::send]
    pub async fn migrate(
        &self,
        env_name: &EnvName,
        state: MigrationState,
    ) -> anyhow::Result<ExistingMigrationState> {
        let (mut version, base_id) = match state {
            MigrationState::New(NewMigrationState {
                title,
                initial_user_email,
            }) => {
                let version = migrations::Version::INITIAL;

                let base_id = create_base(self.noco_client, title, initial_user_email).await?;

                kv::put_base_id(self.kv, env_name, &base_id).await?;

                self.neon_client
                    .create_backup(
                        &env_name.clone().into(),
                        &noco_migration_branch_name(&version),
                    )
                    .await?;

                (version, base_id)
            }
            MigrationState::Existing(ExistingMigrationState { version, base_id }) => {
                (version, base_id)
            }
        };

        loop {
            let is_up_to_date = self
                .neon_client
                .with_rollback(
                    &env_name.clone().into(),
                    &NOCO_PRE_MIGRATION_ROLLBACK_BRANCH_NAME,
                    async || {
                        match migrations::run(self.noco_client, base_id.clone(), version.next())
                            .await
                        {
                            Ok(migrations::Outcome::AlreadyUpToDate) => return Ok(true),
                            Err(error) => {
                                console_error!(
                                    "Migration {} failed. Rolling back.",
                                    version.next()
                                );
                                return Err(error);
                            }
                            _ => {}
                        }

                        version = version.next();

                        kv::put_migration_version(self.kv, env_name, version).await?;

                        Ok(false)
                    },
                )
                .await?;

            if is_up_to_date {
                break;
            }

            self.neon_client
                .create_backup(
                    &env_name.clone().into(),
                    &noco_migration_branch_name(&version),
                )
                .await?;
        }

        Ok(ExistingMigrationState { base_id, version })
    }
}
