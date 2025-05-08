use axum::{Json, Router, response::ErrorResponse, routing::post};
use reqwest::{StatusCode, Url};
use worker::console_error;

use crate::{
    api::{PostBaseRequest, PostBaseResponse},
    config,
    noco::{self, ApiToken},
};

fn to_status<T: Into<anyhow::Error>>(code: StatusCode) -> impl FnOnce(T) -> ErrorResponse {
    move |err| {
        console_error!("Error: {}", err.into());
        code.into()
    }
}

pub fn new() -> Router {
    Router::new().route("/bases", post(post_base))
}

#[axum::debug_handler]
async fn post_base(
    Json(body): Json<PostBaseRequest>,
) -> Result<Json<PostBaseResponse>, ErrorResponse> {
    let base_domain = config::base_domain();
    let app_origin = Url::parse(&format!("https://{}.{}", body.app_domain, base_domain))
        .map_err(to_status(StatusCode::BAD_REQUEST))?;

    let client = noco::Client::new(app_origin, ApiToken::from(body.api_token));

    let base_url = client
        .setup_base(body.title)
        .await
        .map_err(to_status(StatusCode::INTERNAL_SERVER_ERROR))?;

    Ok(Json(PostBaseResponse {
        url: base_url.to_string(),
    }))
}
