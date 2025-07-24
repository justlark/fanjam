mod common;
mod n1;

// Each base schema migration lives in its own module with the name `nX`, where `X` is the
// incrementing migration number.

pub use super::client::Client;
pub use common::{BaseId, Migration, TableId, Version};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    AlreadyUpToDate,
    Migrated,
}

// New migrations must added to the list here to be applied.
pub async fn run(client: &Client, base_id: BaseId, version: Version) -> anyhow::Result<Outcome> {
    match version {
        n1::Migration::INDEX => n1::Migration::new(client).migrate(base_id).await?,
        // Example:
        // n2::Migration::INDEX => n2::Migration::new(client).migrate(base_id).await?,
        _ => return Ok(Outcome::AlreadyUpToDate),
    }

    Ok(Outcome::Migrated)
}
