use serde_json::json;

use crate::noco::{
    BaseId, Client, TableIds, Version, list_tables,
    migrations::{
        common::{self, CreateColumnRequest, create_columns, set_nop},
        n06,
    },
};

pub struct Migration<'a> {
    client: &'a Client,
    ctx: &'a common::MigrationContext,
}

impl Migration<'_> {
    async fn create_columns(&self, table_ids: &TableIds) -> anyhow::Result<()> {
        let requests = vec![CreateColumnRequest {
            table_id: &table_ids.locations,
            column_ref: set_nop(),
            body: json!({
                "column_name": "description",
                "title": "Description",
                "uidt": "LongText",
                "description": "Information about this location.",
                "meta": {
                    "richMode": true
                }
            }),
        }];

        create_columns(self.client, requests, (&self.ctx.env_name).into()).await?;

        Ok(())
    }
}

impl<'a> common::Migration<'a> for Migration<'a> {
    const INDEX: Version = n06::Migration::INDEX.next();

    fn new(client: &'a Client, ctx: &'a common::MigrationContext) -> Self {
        Self { client, ctx }
    }

    async fn migrate(&self, base_id: BaseId) -> anyhow::Result<()> {
        let tables = TableIds::try_from(list_tables(self.client, &base_id).await?)?;

        self.create_columns(&tables).await?;

        Ok(())
    }
}
