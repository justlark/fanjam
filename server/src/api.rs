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
    pub local_url: String,
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
    pub summary: Option<String>,
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
pub struct Announcement {
    pub id: String,
    pub title: String,
    pub body: String,
    pub attachments: Vec<File>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize)]
pub struct GetAnnouncementsResponse {
    pub announcements: Vec<Announcement>,
}

#[derive(Debug, Serialize)]
pub struct Page {
    pub id: String,
    pub title: String,
    pub body: String,
    pub files: Vec<File>,
}

#[derive(Debug, Serialize)]
pub struct GetPagesResponse {
    pub pages: Vec<Page>,
}

#[derive(Debug, Serialize)]
pub struct GetSummaryResponse {
    pub env_name: String,
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct GetConfigResponse {
    pub timezone: Option<String>,
    pub feedback_url: Option<String>,
    pub use_custom_icon: Option<bool>,
    pub favicon_name: Option<String>,
    pub opengraph_icon_name: Option<String>,
    pub opengraph_icon_type: Option<String>,
    pub opengraph_icon_alt: Option<String>,
    pub pwa_short_app_name: Option<String>,
    pub pwa_background_color: Option<String>,
    pub pwa_icon_any_name: Option<String>,
    pub pwa_icon_any_type: Option<String>,
    pub pwa_icon_any_sizes: Option<String>,
    pub pwa_icon_maskable_name: Option<String>,
    pub pwa_icon_maskable_type: Option<String>,
    pub pwa_icon_maskable_sizes: Option<String>,
}
