mod common;
mod n01;
mod n02;
mod n03;
mod n04;
mod n05;
mod n06;
mod n07;
mod n08;
mod n09;
mod n10;
mod n11;

// Each base schema migration lives in its own module with the name `nX`, where `X` is the
// incrementing migration number.

pub use super::client::Client;
pub use common::{
    BaseId, Migration, MigrationContext, TableId, TableIds, TableInfo, Version, list_tables,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    AlreadyUpToDate,
    Migrated,
}

// New migrations must be added to the list here to be applied.
pub async fn run(
    client: &Client,
    base_id: BaseId,
    version: Version,
    ctx: &MigrationContext,
) -> anyhow::Result<Outcome> {
    match version {
        n01::Migration::INDEX => n01::Migration::new(client, ctx).migrate(base_id).await?,
        n02::Migration::INDEX => n02::Migration::new(client, ctx).migrate(base_id).await?,
        n03::Migration::INDEX => n03::Migration::new(client, ctx).migrate(base_id).await?,
        n04::Migration::INDEX => n04::Migration::new(client, ctx).migrate(base_id).await?,
        n05::Migration::INDEX => n05::Migration::new(client, ctx).migrate(base_id).await?,
        n06::Migration::INDEX => n06::Migration::new(client, ctx).migrate(base_id).await?,
        n07::Migration::INDEX => n07::Migration::new(client, ctx).migrate(base_id).await?,
        n08::Migration::INDEX => n08::Migration::new(client, ctx).migrate(base_id).await?,
        n09::Migration::INDEX => n09::Migration::new(client, ctx).migrate(base_id).await?,
        n10::Migration::INDEX => n10::Migration::new(client, ctx).migrate(base_id).await?,
        n11::Migration::INDEX => n11::Migration::new(client, ctx).migrate(base_id).await?,
        _ => return Ok(Outcome::AlreadyUpToDate),
    }

    Ok(Outcome::Migrated)
}
