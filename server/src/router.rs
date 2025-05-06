use axum::{Json, Router, response::ErrorResponse, routing::post};

use crate::api::PostBaseResponse;

pub fn new() -> Router {
    Router::new().route("/bases", post(post_base))
}

#[axum::debug_handler]
async fn post_base() -> Result<Json<PostBaseResponse>, ErrorResponse> {
    todo!()
}
