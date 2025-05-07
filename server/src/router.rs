use axum::{Json, Router, response::ErrorResponse, routing::post};
use reqwest::StatusCode;
use worker::console_error;

use crate::{
    api::{PostBaseRequest, PostBaseResponse},
    noco,
};

fn internal_err(err: anyhow::Error) -> ErrorResponse {
    console_error!("Error: {}", err);
    StatusCode::INTERNAL_SERVER_ERROR.into()
}

pub fn new() -> Router {
    Router::new().route("/bases", post(post_base))
}

#[axum::debug_handler]
async fn post_base(
    Json(body): Json<PostBaseRequest>,
) -> Result<Json<PostBaseResponse>, ErrorResponse> {
    let client = noco::Client::new();

    let base_url = client.setup_base(body.title).await.map_err(internal_err)?;

    Ok(Json(PostBaseResponse {
        url: base_url.to_string(),
    }))
}
