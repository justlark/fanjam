use serde::{Deserialize, de::DeserializeOwned};
use worker::Method;

use crate::noco::Client;

use super::{BaseId, migrations::TableId};

async fn list_records<T: DeserializeOwned>(
    client: &Client,
    table_id: TableId,
) -> anyhow::Result<Vec<T>> {
    #[derive(Debug, Deserialize)]
    struct GetRecordsResponse<T> {
        list: Vec<T>,
    }

    // TODO: Implement pagination logic.
    Ok(client
        .build_request(Method::Get, &format!("/tables/{table_id}/records"))
        .with_param("limit", "100")
        .fetch::<GetRecordsResponse<T>>()
        .await?
        .list)
}

#[derive(Debug, Deserialize)]
struct TableInfo {
    id: String,
    table_name: String,
}

async fn list_tables(client: &Client, base_id: &BaseId) -> anyhow::Result<Vec<TableInfo>> {
    #[derive(Debug, Deserialize)]
    struct GetTablesResponse {
        list: Vec<TableInfo>,
    }

    Ok(client
        .build_request(Method::Get, &format!("/meta/bases/{base_id}/tables"))
        .fetch::<GetTablesResponse>()
        .await?
        .list)
}

#[derive(Debug)]
pub struct TableIds {
    pub events: TableId,
}

fn find_in_tables(tables: &[TableInfo], table_name: &str) -> anyhow::Result<TableId> {
    tables
        .iter()
        .find(|table| table.table_name == table_name)
        .map(|table| TableId::from(table.id.clone()))
        .ok_or_else(|| anyhow::anyhow!("No table named `{table_name}` found"))
}

async fn find_tables(client: &Client, base_id: &BaseId) -> anyhow::Result<TableIds> {
    let table_info = list_tables(client, base_id).await?;

    if table_info.is_empty() {
        return Err(anyhow::anyhow!("No tables found in base `{base_id}`"));
    }

    Ok(TableIds {
        events: find_in_tables(&table_info, "events")?,
    })
}

#[derive(Debug, Deserialize)]
struct LocationResponse {
    #[serde(rename = "Location")]
    pub name: String,
}

#[derive(Debug, Deserialize)]
struct CategoryResponse {
    #[serde(rename = "Category")]
    pub name: String,
}

#[derive(Debug, Deserialize)]
struct EventResponse {
    #[serde(rename = "ID")]
    pub id: u32,
    #[serde(rename = "Event Name")]
    pub name: String,
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(rename = "Start Time")]
    pub start_time: String,
    #[serde(rename = "End Time")]
    pub end_time: Option<String>,
    #[serde(rename = "Locations")]
    pub location: Option<LocationResponse>,
    #[serde(rename = "Categories")]
    pub category: Option<CategoryResponse>,
    #[serde(rename = "Hidden")]
    pub hidden: bool,
}

#[derive(Debug)]
pub struct Event {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub start_time: String,
    pub end_time: Option<String>,
    pub location: Option<String>,
    pub category: Option<String>,
}

#[worker::send]
pub async fn list_events(client: &Client, base_id: &BaseId) -> anyhow::Result<Vec<Event>> {
    let table_ids = find_tables(client, base_id).await?;

    let records = list_records::<EventResponse>(client, table_ids.events).await?;

    Ok(records
        .into_iter()
        .filter(|r| !r.hidden)
        .map(|r| Event {
            id: r.id.to_string(),
            name: r.name,
            description: r.description,
            start_time: r.start_time,
            end_time: r.end_time,
            location: r.location.map(|l| l.name),
            category: r.category.map(|c| c.name),
        })
        .collect())
}
