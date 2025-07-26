use serde_json::json;

use crate::noco::migrations::common::TableRequest;

use super::{
    BaseId, Client, TableId, Version,
    common::{self, ColumnRequest, create_columns, create_tables, set_nop, set_ref},
    n1,
};

#[derive(Debug, Default)]
struct ByTable<T> {
    pages: T,
}

type Tables = ByTable<TableId>;

impl<T> ByTable<T> {
    fn map<U>(self, f: impl Fn(T) -> U) -> ByTable<U> {
        ByTable {
            pages: f(self.pages),
        }
    }
}

pub struct Migration<'a> {
    client: &'a Client,
}

impl Migration<'_> {
    async fn create_tables(&self, base_id: &BaseId) -> anyhow::Result<Tables> {
        let mut tables = ByTable::<Option<TableId>>::default();

        let requests = vec![TableRequest {
            body: json!({
                "table_name": "pages",
                "title": "Pages",
                "description": "Add additional pages to the app with information about your con.",
                "meta": {
                    "icon": "✏️"
                },
                "columns": [
                    {
                        "column_name": "id",
                        "title": "ID",
                        "uidt": "ID"
                    },
                    {
                        "column_name": "title",
                        "title": "Page Title",
                        "uidt": "SingleLineText",
                        "description": "The title of the page.",
                        "pv": true,
                        "rqd": true,
                    }
                ]
            }),
            table_ref: set_ref(&mut tables.pages),
        }];

        create_tables(self.client, base_id, requests).await?;

        Ok(tables.map(|id| id.expect("expected table ID, found none")))
    }

    async fn create_columns(&self, tables: &Tables) -> anyhow::Result<()> {
        let requests = vec![ColumnRequest {
            table_id: &tables.pages,
            column_ref: set_nop(),
            body: json!({
                "column_name": "body",
                "title": "Page Body",
                "uidt": "LongText",
                "description": "The contents of the page.",
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
    const INDEX: Version = n1::Migration::INDEX.next();

    fn new(client: &'a Client) -> Self {
        Self { client }
    }

    async fn migrate(&self, base_id: BaseId) -> anyhow::Result<()> {
        let tables = self.create_tables(&base_id).await?;
        self.create_columns(&tables).await?;

        Ok(())
    }
}
