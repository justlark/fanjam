use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct PostBaseRequest {
    pub title: String,
    pub dash_domain: String,
    pub api_token: String,
}

#[derive(Debug, Serialize)]
pub struct PostBaseResponse {
    pub dash_url: String,
    pub app_url: String,
}
