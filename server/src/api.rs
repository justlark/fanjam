use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct PostBaseRequest {
    pub title: String,
}

#[derive(Debug, Serialize)]
pub struct PostBaseResponse {
    pub url: String,
}
