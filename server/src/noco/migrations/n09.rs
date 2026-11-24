use serde_json::json;

use crate::noco::{
    BaseId, Client, Version,
    migrations::{
        common::{
            self, CreateColumnRequest, TableId, TableRequest, create_columns, create_tables,
            set_nop, set_ref,
        },
        n08,
    },
};

#[derive(Debug, Default)]
struct ByTable<T> {
    vendors: T,
    vendor_pages: T,
}

type Tables = ByTable<TableId>;

impl<T> ByTable<T> {
    fn map<U>(self, f: impl Fn(T) -> U) -> ByTable<U> {
        ByTable {
            vendors: f(self.vendors),
            vendor_pages: f(self.vendor_pages),
        }
    }
}
pub struct Migration<'a> {
    client: &'a Client,
    ctx: &'a common::MigrationContext,
}

impl Migration<'_> {
    async fn create_tables(&self, base_id: &BaseId) -> anyhow::Result<Tables> {
        let mut tables = ByTable::<Option<TableId>>::default();

        let requests = vec![
            TableRequest {
                body: json!({
                    "table_name": "vendors",
                    "title": "Vendors",
                    "description": "A list of vendors, sponsors, organizations, dealers, artists, or any other groups you want to call out.",
                    "meta": {
                        "icon": "💼"
                    },
                    "columns": [
                        {
                            "column_name": "id",
                            "title": "ID",
                            "uidt": "ID"
                        },
                        {
                            "column_name": "name",
                            "title": "Vendor Name",
                            "uidt": "SingleLineText",
                            "description": "The name of the vendor.",
                            "pv": true,
                            "rqd": true,
                        }
                    ]
                }),
                table_ref: set_ref(&mut tables.vendors),
            },
            TableRequest {
                body: json!({
                    "table_name": "vendor_pages",
                    "title": "Vendor Pages",
                    "description": "Pages for grouping different kinds of vendors.",
                    "meta": {
                        "icon": "🧭"
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
                            "description": "The page title.",
                            "pv": true,
                            "rqd": true,
                        }
                    ]
                }),
                table_ref: set_ref(&mut tables.vendor_pages),
            },
        ];

        create_tables(self.client, base_id, requests, (&self.ctx.env_name).into()).await?;

        Ok(tables.map(|id| id.expect("expected table ID, found none")))
    }

    async fn create_columns(&self, tables: &Tables) -> anyhow::Result<()> {
        let requests = vec![
            CreateColumnRequest {
                table_id: &tables.vendor_pages,
                column_ref: set_nop(),
                body: json!({
                    "column_name": "vendors",
                    "title": "Vendors",
                    "uidt": "Links",
                    "description": "The list of vendors on this page.",
                    "type": "hm",
                    "parentId": &tables.vendor_pages,
                    "childId": &tables.vendors
                }),
            },
            CreateColumnRequest {
                table_id: &tables.vendor_pages,
                column_ref: set_nop(),
                body: json!({
                    "column_name": "description",
                    "title": "Description",
                    "uidt": "LongText",
                    "description": "A description to accompany this vendor page.",
                    "meta": {
                        "richMode": false
                    }
                }),
            },
            CreateColumnRequest {
                table_id: &tables.vendors,
                column_ref: set_nop(),
                body: json!({
                    "column_name": "summary",
                    "title": "Summary",
                    "uidt": "LongText",
                    "description": "A brief summary for this vendor.",
                    "meta": {
                        "richMode": false
                    }
                }),
            },
            CreateColumnRequest {
                table_id: &tables.vendors,
                column_ref: set_nop(),
                body: json!({
                    "column_name": "description",
                    "title": "Description",
                    "uidt": "LongText",
                    "description": "Detailed information about this vendor.",
                    "meta": {
                        "richMode": true
                    }
                }),
            },
            CreateColumnRequest {
                table_id: &tables.vendors,
                column_ref: set_nop(),
                body: json!({
                    "column_name": "website",
                    "title": "Website",
                    "uidt": "URL",
                    "description": "A link to the vendor's website.",
                    "meta": {
                        "validate": true
                    }
                }),
            },
        ];

        create_columns(self.client, requests, (&self.ctx.env_name).into()).await?;

        Ok(())
    }
}

impl<'a> common::Migration<'a> for Migration<'a> {
    const INDEX: Version = n08::Migration::INDEX.next();

    fn new(client: &'a Client, ctx: &'a common::MigrationContext) -> Self {
        Self { client, ctx }
    }

    async fn migrate(&self, base_id: BaseId) -> anyhow::Result<()> {
        let tables = self.create_tables(&base_id).await?;
        self.create_columns(&tables).await?;

        Ok(())
    }
}
