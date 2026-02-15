use std::collections::HashMap;

use chrono::DateTime;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use worker::Method;

use crate::noco::Client;

use super::migrations::{TableId, TableIds};

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
            .build_request_v2(Method::Get, &format!("/tables/{table_id}/records"))
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
    #[serde(rename = "Summary")]
    pub summary: Option<String>,
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(rename = "Start Time")]
    pub start_time: Option<String>,
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
struct TagResponse {
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
    #[serde(rename = "Files")]
    pub files: Option<Vec<FileBodyResponse>>,
}

#[derive(Debug, Deserialize)]
struct LinkResponse {
    #[serde(rename = "Link Name")]
    pub name: String,
    #[serde(rename = "URL")]
    pub url: String,
}

#[derive(Debug, Deserialize)]
struct FileBodyResponse {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "mimetype")]
    pub media_type: String,
    #[serde(rename = "signedUrl")]
    pub signed_url: String,
}

#[derive(Debug, Deserialize)]
struct PageResponse {
    #[serde(rename = "ID")]
    pub id: u32,
    #[serde(rename = "Page Title")]
    pub title: String,
    #[serde(rename = "Page Body")]
    pub body: Option<String>,
    #[serde(rename = "Files")]
    pub files: Option<Vec<FileBodyResponse>>,
}

#[derive(Debug, Deserialize)]
struct AnnouncementResponse {
    #[serde(rename = "ID")]
    pub id: u32,
    #[serde(rename = "Title")]
    pub title: String,
    #[serde(rename = "Announcement")]
    pub body: Option<String>,
    #[serde(rename = "Files")]
    pub files: Option<Vec<FileBodyResponse>>,
    #[serde(rename = "Created")]
    pub creatd_at: String,
    #[serde(rename = "Last Edited")]
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: String,
    pub name: String,
    pub summary: Option<String>,
    pub description: Option<String>,
    pub start_time: String,
    pub end_time: Option<String>,
    pub location: Option<String>,
    pub category: Option<String>,
    pub people: Vec<String>,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct About {
    pub name: Option<String>,
    pub description: Option<String>,
    pub website_url: Option<String>,
    pub files: Vec<File>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Link {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Info {
    pub about: About,
    pub links: Vec<Link>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct File {
    pub id: String,
    pub name: String,
    pub media_type: String,
    pub signed_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Page {
    pub id: String,
    pub title: String,
    pub body: String,
    pub files: Vec<File>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Announcement {
    pub id: String,
    pub title: String,
    pub body: String,
    pub files: Vec<File>,
    pub created_at: String,
    pub updated_at: String,
}

#[worker::send]
pub async fn get_events(client: &Client, table_ids: &TableIds) -> anyhow::Result<Vec<Event>> {
    let (event_records_result, people_records_result, tags_records_result) = futures::join!(
        list_records::<EventResponse>(client, &table_ids.events),
        list_records::<PeopleResponse>(client, &table_ids.people),
        list_records::<TagResponse>(client, &table_ids.tags),
    );

    let people_id_to_name: HashMap<u32, String> = people_records_result?
        .iter()
        .map(|p| (p.id, p.name.clone()))
        .collect();
    let tags_id_to_name: HashMap<u32, String> = tags_records_result?
        .iter()
        .map(|p| (p.id, p.name.clone()))
        .collect();

    let mut events = event_records_result?
        .into_iter()
        .filter(|r| !r.hidden)
        // We allow event organizers to create events in NocoDB without a start time to give them
        // more flexibility in how they plan the schedule. However, events without a start time
        // will not be returned to the client, because it's not obvious how the client should
        // display them in the schedule view.
        //
        // Similarly, we filter out events where the end time comes before the start time, because
        // it's not obvious how the client should display them.
        .filter(|r| match r {
            EventResponse {
                start_time: Some(start_time),
                end_time: Some(end_time),
                ..
            } => match (
                DateTime::parse_from_rfc3339(start_time),
                DateTime::parse_from_rfc3339(end_time),
            ) {
                (Ok(start_time), Ok(end_time)) => start_time <= end_time,
                _ => false,
            },
            EventResponse {
                start_time: Some(_),
                ..
            } => true,
            EventResponse {
                start_time: None, ..
            } => false,
        })
        .map(|r| Event {
            id: r.id.to_string(),
            name: r.name,
            summary: r.summary,
            description: r.description,
            start_time: r.start_time.unwrap(),
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
        .collect::<Vec<_>>();

    // Sort events by start time, then end time.
    events.sort_by(|a, b| a.end_time.cmp(&b.end_time));
    events.sort_by(|a, b| a.start_time.cmp(&b.start_time));

    Ok(events)
}

pub async fn get_about(client: &Client, table_ids: &TableIds) -> anyhow::Result<About> {
    let about_records = list_records::<AboutResponse>(client, &table_ids.about).await?;
    let latest_record = about_records.into_iter().next_back();

    Ok(latest_record
        .map(|r| About {
            name: Some(r.name),
            description: r.description,
            website_url: r.website_url,
            files: r
                .files
                .unwrap_or_default()
                .into_iter()
                .map(|f| File {
                    id: f.id,
                    name: f.title,
                    media_type: f.media_type,
                    signed_url: f.signed_url,
                })
                .collect(),
        })
        .unwrap_or_default())
}

async fn get_links(client: &Client, table_ids: &TableIds) -> anyhow::Result<Vec<Link>> {
    let link_records = list_records::<LinkResponse>(client, &table_ids.links).await?;

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

#[worker::send]
pub async fn get_pages(client: &Client, table_ids: &TableIds) -> anyhow::Result<Vec<Page>> {
    let page_records = list_records::<PageResponse>(client, &table_ids.pages).await?;

    Ok(page_records
        .into_iter()
        .map(|r| Page {
            id: r.id.to_string(),
            title: r.title,
            body: r.body.unwrap_or_default(),
            files: r
                .files
                .unwrap_or_default()
                .into_iter()
                .map(|f| File {
                    id: f.id,
                    name: f.title,
                    media_type: f.media_type,
                    signed_url: f.signed_url,
                })
                .collect(),
        })
        .collect())
}

#[worker::send]
pub async fn get_announcements(
    client: &Client,
    table_ids: &TableIds,
) -> anyhow::Result<Vec<Announcement>> {
    let page_records =
        list_records::<AnnouncementResponse>(client, &table_ids.announcements).await?;

    Ok(page_records
        .into_iter()
        .map(|a| Announcement {
            id: a.id.to_string(),
            title: a.title,
            body: a.body.unwrap_or_default(),
            files: a
                .files
                .unwrap_or_default()
                .into_iter()
                .map(|f| File {
                    id: f.id,
                    name: f.title,
                    media_type: f.media_type,
                    signed_url: f.signed_url,
                })
                .collect(),
            created_at: a.creatd_at,
            updated_at: a.updated_at,
        })
        .collect())
}
