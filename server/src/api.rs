use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct PostBaseResponse {
    pub base_id: String,
}
