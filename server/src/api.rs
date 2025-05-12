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
pub struct PostMigrationResponse {
    pub old_version: noco::Version,
    pub new_version: noco::Version,
}

#[derive(Debug, Serialize)]
pub struct GetMigrationResponse {
    pub version: noco::Version,
}
