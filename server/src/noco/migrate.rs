use worker::console_error;

use crate::{env::EnvName, neon::BackupBranch};

use super::{
    create_base,
    migrations::{self, BaseId, Client as NocoClient, Version},
};
use crate::neon::Client as NeonClient;
use crate::sql::Client as DbClient;

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

#[derive(Debug)]
pub struct Migrator<'a> {
    noco_client: &'a NocoClient,
    neon_client: &'a NeonClient,
    db_client: &'a DbClient,
}

impl<'a> Migrator<'a> {
    pub fn new(
        noco_client: &'a NocoClient,
        neon_client: &'a NeonClient,
        db_client: &'a DbClient,
    ) -> Self {
        Self {
            noco_client,
            neon_client,
            db_client,
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
                let base_id = self
                    .neon_client
                    .with_rollback(
                        &env_name.clone().into(),
                        &BackupBranch::BaseCreationRollback,
                        async || {
                            let base_id =
                                create_base(self.noco_client, title, initial_user_email).await?;
                            self.db_client.set_base(&base_id).await?;
                            Ok(base_id)
                        },
                    )
                    .await?;

                (migrations::Version::INITIAL, base_id)
            }
            MigrationState::Existing(ExistingMigrationState { version, base_id }) => {
                (version, base_id)
            }
        };

        self.neon_client
            .create_backup(
                &env_name.clone().into(),
                crate::neon::BackupBranch::Migration,
            )
            .await?;

        loop {
            let is_up_to_date = self
                .neon_client
                .with_rollback(
                    &env_name.clone().into(),
                    &BackupBranch::MigrationRollback,
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

                        self.db_client.set_migration(&base_id, &version).await?;

                        Ok(false)
                    },
                )
                .await?;

            if is_up_to_date {
                break;
            }
        }

        Ok(ExistingMigrationState { base_id, version })
    }
}
