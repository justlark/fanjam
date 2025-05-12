use super::{
    create_base,
    migrations::{self, BaseId, Client, Version},
};

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
}

// TODO: After each migration, branch the Neon database. Automatically roll back if the next
// migration fails.
//
// TODO: After each migration, store the new migration version number in KV.
#[worker::send]
pub async fn migrate(
    client: &Client,
    state: MigrationState,
) -> anyhow::Result<ExistingMigrationState> {
    let (mut version, base_id) = match state {
        MigrationState::New(NewMigrationState {
            title,
            initial_user_email,
        }) => {
            let base_id = create_base(client, title, initial_user_email).await?;
            (migrations::Version::INITIAL, base_id)
        }
        MigrationState::Existing(ExistingMigrationState { version, base_id }) => (version, base_id),
    };

    while migrations::run(client, base_id.clone(), version.next()).await?
        == migrations::Outcome::Migrated
    {
        version = version.next();
    }

    Ok(ExistingMigrationState { base_id, version })
}
