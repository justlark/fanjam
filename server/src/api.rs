use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct PostBaseRequest {
    pub title: String,
    pub app_domain: String,
    pub api_token: String,
}

#[derive(Debug, Serialize)]
pub struct PostBaseResponse {
    pub url: String,
}
