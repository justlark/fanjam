mod common;
mod noco1;

use common::Migration;
pub use common::{ApiToken, BaseId, Client, Version, init};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    AlreadyUpToDate,
    Migrated,
}

// Add new migrations to the list here.
pub async fn run(client: &Client, base_id: BaseId, version: Version) -> anyhow::Result<Outcome> {
    match version {
        noco1::Migration::INDEX => noco1::Migration::new(client).migrate(base_id).await?,
        _ => return Ok(Outcome::AlreadyUpToDate),
    }

    Ok(Outcome::Migrated)
}
