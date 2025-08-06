use serde_json::json;

use crate::noco::{TableIds, list_tables};

use super::{
    BaseId, Client, Version,
    common::{self, ColumnIds, EditColumnRequest, edit_columns, list_columns},
    n2,
};

pub struct Migration<'a> {
    client: &'a Client,
}

impl Migration<'_> {
    async fn edit_columns(&self, table_ids: &TableIds) -> anyhow::Result<()> {
        let events_columns = ColumnIds::from(list_columns(self.client, &table_ids.events).await?);

        let start_time_column_id = events_columns.find("start_time")?;

        let requests = vec![EditColumnRequest {
            column_id: &start_time_column_id,
            body: json!({
                "rqd": false,
            }),
        }];

        edit_columns(self.client, requests).await?;

        Ok(())
    }
}

impl<'a> common::Migration<'a> for Migration<'a> {
    const INDEX: Version = n2::Migration::INDEX.next();

    fn new(client: &'a Client) -> Self {
        Self { client }
    }

    async fn migrate(&self, base_id: BaseId) -> anyhow::Result<()> {
        let tables = TableIds::try_from(list_tables(self.client, &base_id).await?)?;
        self.edit_columns(&tables).await?;

        Ok(())
    }
}
