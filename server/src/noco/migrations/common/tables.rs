use std::{collections::HashMap, fmt};

use serde::{Deserialize, Serialize};
use worker::{Method, console_log};

use crate::noco::Client;

use super::{BaseId, RefSetter, TableId};

#[derive(Debug, Serialize, Deserialize)]
pub struct TableInfo {
    pub id: TableId,
    #[serde(rename = "table_name")]
    pub name: String,
}

#[worker::send]
pub async fn list_tables(client: &Client, base_id: &BaseId) -> anyhow::Result<Vec<TableInfo>> {
    #[derive(Debug, Deserialize)]
    struct GetTablesResponse {
        list: Vec<TableInfo>,
    }

    Ok(client
        .build_request_v2(Method::Get, &format!("/meta/bases/{base_id}/tables"))
        .fetch::<GetTablesResponse>()
        .await?
        .list)
}

pub struct TableIds {
    pub events: TableId,
    pub people: TableId,
    pub tags: TableId,
    pub about: TableId,
    pub links: TableId,
    pub files: TableId,
    pub pages: TableId,
    pub announcements: TableId,
}

impl TryFrom<Vec<TableInfo>> for TableIds {
    type Error = anyhow::Error;

    fn try_from(tables: Vec<TableInfo>) -> Result<Self, Self::Error> {
        let mut ids = HashMap::new();
        for table in tables {
            ids.insert(table.name, table.id);
        }

        Ok(TableIds {
            events: ids
                .remove("events")
                .ok_or_else(|| anyhow::anyhow!("Missing 'events' table in cache"))?,
            people: ids
                .remove("people")
                .ok_or_else(|| anyhow::anyhow!("Missing 'people' table in cache"))?,
            tags: ids
                .remove("tags")
                .ok_or_else(|| anyhow::anyhow!("Missing 'tags' table in cache"))?,
            about: ids
                .remove("about")
                .ok_or_else(|| anyhow::anyhow!("Missing 'about' table in cache"))?,
            links: ids
                .remove("links")
                .ok_or_else(|| anyhow::anyhow!("Missing 'links' table in cache"))?,
            files: ids
                .remove("files")
                .ok_or_else(|| anyhow::anyhow!("Missing 'files' table in cache"))?,
            pages: ids
                .remove("pages")
                .ok_or_else(|| anyhow::anyhow!("Missing 'pages' table in cache"))?,
            announcements: ids
                .remove("announcements")
                .ok_or_else(|| anyhow::anyhow!("Missing 'announcements' table in cache"))?,
        })
    }
}

pub struct TableRequest<'a> {
    pub body: serde_json::Value,
    pub table_ref: RefSetter<'a, TableId>,
}

impl fmt::Debug for TableRequest<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TableRequest")
            .field("body", &self.body)
            .finish_non_exhaustive()
    }
}

pub async fn create_tables(
    client: &Client,
    base_id: &BaseId,
    requests: Vec<TableRequest<'_>>,
) -> anyhow::Result<()> {
    #[derive(Debug, Deserialize)]
    struct PostTableResponse {
        id: TableId,
    }

    for request in requests {
        let table_id = client
            .build_request_v2(Method::Post, &format!("/meta/bases/{base_id}/tables"))
            .with_json(&request.body)?
            .fetch::<PostTableResponse>()
            .await?
            .id;

        let table_name = request
            .body
            .as_object()
            .and_then(|obj| obj.get("title"))
            .and_then(|title| title.as_str())
            .unwrap_or("Unknown");

        console_log!("Created Noco table `{}` with ID `{}`", table_name, table_id);

        (request.table_ref)(table_id.clone());
    }

    Ok(())
}
