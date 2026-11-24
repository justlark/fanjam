use crate::noco::{
    BaseId, Client, Version,
    migrations::{common, n04},
};

// This migration used to install the push notifications hook, keyed by the env ID. It's superseded
// by n11, which installs the hook keyed by the env name instead and replaces any hook this migration
// installed. Bases already past this version are unaffected.
//
// It's kept as a no-op so that migration numbers stay stable.
pub struct Migration;

impl<'a> common::Migration<'a> for Migration {
    const INDEX: Version = n04::Migration::INDEX.next();

    fn new(_client: &'a Client, _ctx: &'a common::MigrationContext) -> Self {
        Self
    }

    async fn migrate(&self, _base_id: BaseId) -> anyhow::Result<()> {
        Ok(())
    }
}
