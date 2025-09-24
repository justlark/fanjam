use serde_json::json;

use crate::noco::{
    BaseId, Client, TableIds, Version, list_tables,
    migrations::{
        common::{
            self, ColumnIds, CreateColumnRequest, create_columns, delete_columns, delete_tables,
            list_columns, set_nop,
        },
        n3,
    },
};

pub struct Migration<'a> {
    client: &'a Client,
}

impl<'a> Migration<'a> {
    async fn delete_columns(&self, table_ids: &TableIds) -> anyhow::Result<()> {
        let people_columns = ColumnIds::from(list_columns(self.client, &table_ids.people).await?);
        let contact_info_column_id = people_columns.find_by_name("contact_info")?;

        delete_columns(self.client, &[contact_info_column_id]).await?;

        Ok(())
    }

    async fn delete_tables(&self, table_ids: &TableIds) -> anyhow::Result<()> {
        delete_tables(
            self.client,
            &[&table_ids
                .files
                .as_ref()
                .ok_or_else(|| anyhow::anyhow!("Missing 'files' table in cache."))?
                .to_owned()],
        )
        .await?;
        Ok(())
    }

    async fn create_columns(&self, table_ids: &TableIds) -> anyhow::Result<()> {
        let requests = vec![
            CreateColumnRequest {
                table_id: &table_ids.about,
                column_ref: set_nop(),
                body: json!({
                    "column_name": "files",
                    "title": "Files",
                    "uidt": "Attachment",
                    "description": "Attach images or other files for attendees to view or download.",
                }),
            },
            CreateColumnRequest {
                table_id: &table_ids.pages,
                column_ref: set_nop(),
                body: json!({
                    "column_name": "files",
                    "title": "Files",
                    "uidt": "Attachment",
                    "description": "Attach images or other files for attendees to view or download.",
                }),
            },
        ];

        create_columns(self.client, requests).await?;

        Ok(())
    }
}

impl<'a> common::Migration<'a> for Migration<'a> {
    const INDEX: Version = n3::Migration::INDEX.next();

    fn new(client: &'a Client) -> Self {
        Self { client }
    }

    async fn migrate(&self, base_id: BaseId) -> anyhow::Result<()> {
        let tables = TableIds::try_from(list_tables(self.client, &base_id).await?)?;

        self.create_columns(&tables).await?;
        self.delete_columns(&tables).await?;
        self.delete_tables(&tables).await?;

        Ok(())
    }
}
