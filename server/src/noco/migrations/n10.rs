use serde_json::json;

use crate::noco::{
    BaseId, Client, Version,
    migrations::{
        common::{
            self, CreateColumnRequest, TableId, TableRequest, create_columns, create_tables,
            set_nop, set_ref,
        },
        n09,
    },
};

#[derive(Debug, Default)]
struct ByTable<T> {
    passwords: T,
}

type Tables = ByTable<TableId>;

impl<T> ByTable<T> {
    fn map<U>(self, f: impl Fn(T) -> U) -> ByTable<U> {
        ByTable {
            passwords: f(self.passwords),
        }
    }
}

pub struct Migration<'a> {
    client: &'a Client,
}

impl<'a> Migration<'a> {
    async fn create_tables(&self, base_id: &BaseId) -> anyhow::Result<Tables> {
        let mut tables = ByTable::<Option<TableId>>::default();

        let requests = vec![TableRequest {
            body: json!({
                "table_name": "passwords",
                "title": "Passwords",
                "description": "Passwords for controlling access to the app. Any password in this table can be used. If this table is empty, the app is open to everyone.",
                "meta": {
                    "icon": "🔑"
                },
                "columns": [
                    {
                        "column_name": "id",
                        "title": "ID",
                        "uidt": "ID"
                    },
                    {
                        "column_name": "name",
                        "title": "Name",
                        "uidt": "SingleLineText",
                        "description": "Use this to remember who has each password or what they're used for.",
                        "pv": true,
                        "rqd": true,
                    }
                ]
            }),
            table_ref: set_ref(&mut tables.passwords),
        }];

        create_tables(self.client, base_id, requests).await?;

        Ok(tables.map(|id| id.expect("expected table ID, found none")))
    }
    async fn create_columns(&self, tables: &Tables) -> anyhow::Result<()> {
        let requests = vec![CreateColumnRequest {
            table_id: &tables.passwords,
            column_ref: set_nop(),
            body: json!({
                "column_name": "password",
                "title": "Password",
                "uidt": "SingleLineText",
                "description": "The password.",
                "rqd": true,
            }),
        }];

        create_columns(self.client, requests).await?;

        Ok(())
    }
}

impl<'a> common::Migration<'a> for Migration<'a> {
    const INDEX: Version = n09::Migration::INDEX.next();

    fn new(client: &'a Client, _ctx: &'a common::MigrationContext) -> Self {
        Self { client }
    }

    async fn migrate(&self, base_id: BaseId) -> anyhow::Result<()> {
        let tables = self.create_tables(&base_id).await?;
        self.create_columns(&tables).await?;

        Ok(())
    }
}
