use serde::{Deserialize, Serialize};

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
