use std::collections::HashMap;

use serde::{Deserialize, Serialize, de::DeserializeOwned};
use worker::{Method, console_log};

use crate::noco::Client;

use super::{BaseId, migrations::TableId};

const PAGE_SIZE: u32 = 100;

#[derive(Debug, Deserialize)]
struct PageInfo {
    #[serde(rename = "isLastPage")]
    is_last_page: bool,
}

async fn list_records<T: DeserializeOwned>(
    client: &Client,
    table_id: &TableId,
) -> anyhow::Result<Vec<T>> {
    #[derive(Debug, Deserialize)]
    struct GetRecordsResponse<T> {
        list: Vec<T>,
        #[serde(rename = "pageInfo")]
        page_info: PageInfo,
    }

    let mut records = Vec::<T>::new();
    let mut offset = 0;

    loop {
        let response = client
            .build_request(Method::Get, &format!("/tables/{table_id}/records"))
            .with_param("limit", &PAGE_SIZE.to_string())
            .with_param("offset", &offset.to_string())
            .fetch::<GetRecordsResponse<T>>()
            .await?;

        offset += response.list.len();
        records.extend(response.list);

        if response.page_info.is_last_page {
            break;
        }
    }

    Ok(records)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TableInfo {
    id: TableId,
    #[serde(rename = "table_name")]
    name: String,
}

#[worker::send]
pub async fn list_tables(client: &Client, base_id: &BaseId) -> anyhow::Result<Vec<TableInfo>> {
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
    #[serde(rename = "_nc_m2m_tags_events")]
    pub tags_m2m: Vec<TagsM2mResponse>,
    #[serde(rename = "_nc_m2m_people_events")]
    pub people_m2m: Vec<PeopleM2mResponse>,
}

#[derive(Debug, Deserialize)]
struct PeopleM2mResponse {
    #[serde(rename = "people_id")]
    pub id: u32,
}

#[derive(Debug, Deserialize)]
struct TagsM2mResponse {
    #[serde(rename = "tags_id")]
    pub id: u32,
}

#[derive(Debug, Deserialize)]
struct PeopleResponse {
    #[serde(rename = "ID")]
    pub id: u32,
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Debug, Deserialize)]
struct TagsResponse {
    #[serde(rename = "ID")]
    pub id: u32,
    #[serde(rename = "Tag")]
    pub name: String,
}

#[derive(Debug, Deserialize)]
struct AboutResponse {
    #[serde(rename = "Con Name")]
    pub name: String,
    #[serde(rename = "Con Description")]
    pub description: Option<String>,
    #[serde(rename = "Website")]
    pub website_url: Option<String>,
}

#[derive(Debug, Deserialize)]
struct LinksResponse {
    #[serde(rename = "Link Name")]
    pub name: String,
    #[serde(rename = "URL")]
    pub url: String,
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
    pub people: Vec<String>,
    pub tags: Vec<String>,
}

#[derive(Debug)]
pub struct About {
    pub name: String,
    pub description: Option<String>,
    pub website_url: Option<String>,
}

#[derive(Debug)]
pub struct Link {
    pub name: String,
    pub url: String,
}

#[derive(Debug)]
pub struct Info {
    pub about: Option<About>,
    pub links: Vec<Link>,
}

pub struct TableIds {
    events: TableId,
    people: TableId,
    tags: TableId,
    about: TableId,
    links: TableId,
}

impl TryFrom<Vec<TableInfo>> for TableIds {
    type Error = anyhow::Error;

    fn try_from(tables: Vec<TableInfo>) -> Result<Self, Self::Error> {
        console_log!("{:?}", &tables);

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
        })
    }
}

#[worker::send]
pub async fn get_events(client: &Client, table_ids: &TableIds) -> anyhow::Result<Vec<Event>> {
    let event_records = list_records::<EventResponse>(client, &table_ids.events).await?;
    let people_records = list_records::<PeopleResponse>(client, &table_ids.people).await?;
    let tags_records = list_records::<TagsResponse>(client, &table_ids.tags).await?;

    let people_id_to_name: HashMap<u32, String> = people_records
        .iter()
        .map(|p| (p.id, p.name.clone()))
        .collect();
    let tags_id_to_name: HashMap<u32, String> = tags_records
        .iter()
        .map(|p| (p.id, p.name.clone()))
        .collect();

    Ok(event_records
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
            people: r
                .people_m2m
                .into_iter()
                .filter_map(|p| people_id_to_name.get(&p.id).cloned())
                .collect(),
            tags: r
                .tags_m2m
                .into_iter()
                .filter_map(|p| tags_id_to_name.get(&p.id).cloned())
                .collect(),
        })
        .collect())
}

async fn get_about(client: &Client, table_ids: &TableIds) -> anyhow::Result<Option<About>> {
    let about_records = list_records::<AboutResponse>(client, &table_ids.about).await?;
    let latest_record = about_records.into_iter().next_back();

    Ok(latest_record.map(|r| About {
        name: r.name,
        description: r.description,
        website_url: r.website_url,
    }))
}

async fn get_links(client: &Client, table_ids: &TableIds) -> anyhow::Result<Vec<Link>> {
    let link_records = list_records::<LinksResponse>(client, &table_ids.links).await?;

    Ok(link_records
        .into_iter()
        .map(|r| Link {
            name: r.name,
            url: r.url,
        })
        .collect())
}

#[worker::send]
pub async fn get_info(client: &Client, table_ids: &TableIds) -> anyhow::Result<Info> {
    let about = get_about(client, table_ids).await?;
    let links = get_links(client, table_ids).await?;

    Ok(Info { about, links })
}
