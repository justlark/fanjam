use serde_json::json;

use crate::noco::migrations::common::{create_tables, set_nop};

use super::{
    BaseId, Client, Version,
    common::{self, TableRequest},
    n1,
};

pub struct Migration<'a> {
    client: &'a Client,
}

impl<'a> common::Migration<'a> for Migration<'a> {
    const INDEX: Version = n1::Migration::INDEX.next();

    fn new(client: &'a Client) -> Self {
        Self { client }
    }

    async fn migrate(&self, base_id: BaseId) -> anyhow::Result<()> {
        let requests = vec![TableRequest {
            body: json!({
                "title": "New Table",
                "description": "A new table!",
                "fields": []
            }),
            table_ref: set_nop(),
        }];

        create_tables(self.client, &base_id, requests).await?;

        Err(anyhow::anyhow!("Migration {} should fail!", Self::INDEX))
    }
}
