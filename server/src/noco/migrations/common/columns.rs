use std::{collections::HashMap, fmt};

use serde::Deserialize;
use worker::{Method, console_log};

use crate::noco::Client;

use super::{ColumnId, RefSetter, TableId};

#[derive(Debug, Deserialize)]
pub struct ColumnInfo {
    pub id: ColumnId,
    #[serde(rename = "column_name")]
    pub name: Option<String>,
    pub title: Option<String>,
}

pub struct ColumnIds {
    by_name: HashMap<String, ColumnId>,
    by_title: HashMap<String, ColumnId>,
}

impl From<Vec<ColumnInfo>> for ColumnIds {
    fn from(info: Vec<ColumnInfo>) -> Self {
        Self {
            by_name: info
                .iter()
                .filter_map(|col| col.name.clone().map(|name| (name, col.id.clone())))
                .collect(),
            by_title: info
                .iter()
                .filter_map(|col| col.title.clone().map(|title| (title, col.id.clone())))
                .collect(),
        }
    }
}

impl ColumnIds {
    pub fn find_by_name(&self, name: &str) -> anyhow::Result<ColumnId> {
        self.by_name
            .get(name)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("Column with column name `{name}` not found"))
    }

    pub fn find_by_title(&self, title: &str) -> anyhow::Result<ColumnId> {
        self.by_title
            .get(title)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("Column with title `{title}` not found"))
    }
}

#[worker::send]
pub async fn list_columns(client: &Client, table_id: &TableId) -> anyhow::Result<Vec<ColumnInfo>> {
    #[derive(Debug, Deserialize)]
    struct GetTableMetadataResponse {
        columns: Vec<ColumnInfo>,
    }

    Ok(client
        .build_request_v2(Method::Get, &format!("/meta/tables/{table_id}"))
        .fetch::<GetTableMetadataResponse>()
        .await?
        .columns)
}

pub struct CreateColumnRequest<'a> {
    pub table_id: &'a TableId,
    pub column_ref: RefSetter<'a, ColumnId>,
    pub body: serde_json::Value,
}

impl fmt::Debug for CreateColumnRequest<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FieldRequest")
            .field("table_id", &self.table_id)
            .field("body", &self.body)
            .finish_non_exhaustive()
    }
}

pub async fn create_columns(
    client: &Client,
    requests: Vec<CreateColumnRequest<'_>>,
) -> anyhow::Result<()> {
    #[derive(Debug, Deserialize)]
    struct PostColumnResponse {
        id: ColumnId,
    }

    for request in requests {
        let column_id = client
            .build_request_v2(
                Method::Post,
                &format!("/meta/tables/{}/columns", request.table_id),
            )
            .with_json(&request.body)?
            .fetch::<PostColumnResponse>()
            .await?
            .id;

        let column_name = request
            .body
            .as_object()
            .and_then(|obj| obj.get("title"))
            .and_then(|title| title.as_str())
            .unwrap_or("Unknown");

        (request.column_ref)(column_id.clone());

        console_log!(
            "Created Noco column `{}` with ID `{}` on table `{}`",
            column_name,
            column_id,
            request.table_id,
        );
    }

    Ok(())
}

#[derive(Debug)]
pub struct EditColumnRequest<'a> {
    pub column_id: &'a ColumnId,
    pub body: serde_json::Value,
}

pub async fn edit_columns(
    client: &Client,
    requests: Vec<EditColumnRequest<'_>>,
) -> anyhow::Result<()> {
    for request in requests {
        client
            .build_request_v2(
                Method::Patch,
                &format!("/meta/columns/{}", request.column_id),
            )
            .with_json(&request.body)?
            .exec()
            .await?;

        console_log!("Edited Noco column with ID `{}`", request.column_id,);
    }

    Ok(())
}

pub async fn delete_columns(client: &Client, column_ids: &[ColumnId]) -> anyhow::Result<()> {
    for column_id in column_ids {
        client
            .build_request_v2(Method::Delete, &format!("/meta/columns/{}", column_id))
            .exec()
            .await?;

        console_log!("Deleted Noco column with ID `{}`", column_id,);
    }

    Ok(())
}
