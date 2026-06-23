use serde_json::json;

use crate::noco::{
    BaseId, Client, TableIds, Version, list_tables,
    migrations::{
        common::{self, CreateColumnRequest, create_columns, set_nop},
        n5,
    },
};

pub struct Migration<'a> {
    client: &'a Client,
}

impl Migration<'_> {
    async fn create_columns(&self, table_ids: &TableIds) -> anyhow::Result<()> {
        let requests = vec![CreateColumnRequest {
            table_id: &table_ids.people,
            column_ref: set_nop(),
            body: json!({
                "column_name": "bio",
                "title": "Bio",
                "uidt": "LongText",
                "description": "Information about this person. Bio, pronouns, contact info, etc. Attendees can read this in the app.",
                "meta": {
                    "richMode": true
                }
            }),
        }];

        create_columns(self.client, requests).await?;

        Ok(())
    }
}

impl<'a> common::Migration<'a> for Migration<'a> {
    const INDEX: Version = n5::Migration::INDEX.next();

    fn new(client: &'a Client, _ctx: &'a common::MigrationContext) -> Self {
        Self { client }
    }

    async fn migrate(&self, base_id: BaseId) -> anyhow::Result<()> {
        let tables = TableIds::try_from(list_tables(self.client, &base_id).await?)?;

        self.create_columns(&tables).await?;

        Ok(())
    }
}
