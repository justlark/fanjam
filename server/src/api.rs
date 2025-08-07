use serde::{Deserialize, Serialize};

use crate::noco;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

#[derive(Debug, Serialize)]
pub struct PostLinkResponse {
    pub dash_url: String,
    pub app_url: String,
}

#[derive(Debug, Serialize)]
pub struct GetLinkResponse {
    pub dash_url: String,
    pub app_url: String,
}

#[derive(Debug, Deserialize)]
pub struct PutTokenRequest {
    pub token: String,
}

#[derive(Debug, Deserialize)]
pub struct PostBaseRequest {
    pub title: String,
    pub email: String,
}

#[derive(Debug, Serialize)]
pub struct PostApplyMigrationResponse {
    pub old_version: noco::Version,
    pub new_version: noco::Version,
}

#[derive(Debug, Serialize)]
pub struct GetCurrentMigrationResponse {
    pub version: noco::Version,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PostBackupKind {
    Deployment,
}

#[derive(Debug, Deserialize)]
pub struct PostBackupRequest {
    #[serde(rename = "type")]
    pub kind: PostBackupKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PostRestoreBackupKind {
    Deletion,
    Deployment,
    Migration,
}

#[derive(Debug, Deserialize)]
pub struct PostRestoreBackupRequest {
    #[serde(rename = "type")]
    pub kind: PostRestoreBackupKind,
}

#[derive(Debug, Serialize)]
pub struct DataResponseEnvelope<T> {
    pub retry_after_ms: Option<u64>,
    pub value: T,
}

#[derive(Debug, Serialize)]
pub struct Event {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub start_time: String,
    pub end_time: Option<String>,
    pub location: Option<String>,
    pub people: Vec<String>,
    pub category: Option<String>,
    pub tags: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct GetEventsResponse {
    pub events: Vec<Event>,
}

#[derive(Debug, Serialize)]
pub struct Link {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Serialize)]
pub struct File {
    pub name: String,
    pub media_type: String,
    pub signed_url: String,
}

#[derive(Debug, Serialize)]
pub struct GetInfoResponse {
    pub name: Option<String>,
    pub description: Option<String>,
    pub website_url: Option<String>,
    pub links: Vec<Link>,
    pub files: Vec<File>,
}

#[derive(Debug, Serialize)]
pub struct Page {
    pub id: String,
    pub title: String,
    pub body: String,
}

#[derive(Debug, Serialize)]
pub struct GetPagesResponse {
    pub pages: Vec<Page>,
}

#[derive(Debug, Serialize)]
pub struct GetSummaryResponse {
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct GetConfigResponse {
    pub timezone: Option<String>,
}
