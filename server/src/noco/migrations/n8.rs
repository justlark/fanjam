use serde_json::json;

use crate::noco::{
    BaseId, Client, TableIds, Version, list_tables,
    migrations::{
        common::{
            self, ColumnIds, CreateColumnRequest, EditColumnRequest, create_columns, edit_columns,
            list_columns, set_nop,
        },
        n7,
    },
};

pub struct Migration<'a> {
    client: &'a Client,
}

impl Migration<'_> {
    // Earlier migrations tried to enable URL validation with `meta.validation`, but the key NocoDB
    // actually reads is `meta.validate`.
    async fn edit_columns(&self, table_ids: &TableIds) -> anyhow::Result<()> {
        let about_columns = ColumnIds::from(list_columns(self.client, &table_ids.about).await?);
        let links_columns = ColumnIds::from(list_columns(self.client, &table_ids.links).await?);

        let website_column_id = about_columns.find_by_name("website")?;
        let url_column_id = links_columns.find_by_name("url")?;

        // We specify the column titles again here because NocoDB can drop them otherwise (see
        // n3).
        let requests = vec![
            EditColumnRequest {
                column_id: &website_column_id,
                body: json!({
                    "title": "Website",
                    "meta": {
                        "validate": true
                    },
                }),
            },
            EditColumnRequest {
                column_id: &url_column_id,
                body: json!({
                    "title": "URL",
                    "meta": {
                        "validate": true
                    },
                }),
            },
        ];

        edit_columns(self.client, requests).await?;

        Ok(())
    }

    async fn create_columns(&self, table_ids: &TableIds) -> anyhow::Result<()> {
        let requests = vec![CreateColumnRequest {
            table_id: &table_ids.locations,
            column_ref: set_nop(),
            body: json!({
                "column_name": "map_link",
                "title": "Map Link",
                "uidt": "URL",
                "description": "A link to this location on a map.",
                "meta": {
                    "validate": true
                },
            }),
        }];

        create_columns(self.client, requests).await?;

        Ok(())
    }
}

impl<'a> common::Migration<'a> for Migration<'a> {
    const INDEX: Version = n7::Migration::INDEX.next();

    fn new(client: &'a Client, _ctx: &'a common::MigrationContext) -> Self {
        Self { client }
    }

    async fn migrate(&self, base_id: BaseId) -> anyhow::Result<()> {
        let tables = TableIds::try_from(list_tables(self.client, &base_id).await?)?;

        self.edit_columns(&tables).await?;
        self.create_columns(&tables).await?;

        Ok(())
    }
}
