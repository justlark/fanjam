use std::{fmt, sync::Arc};

use axum::{Json, Router, extract::State, response::ErrorResponse, routing::post};
use reqwest::StatusCode;
use worker::{console_error, kv::KvStore};

use crate::{
    api::{PostBaseRequest, PostBaseResponse},
    env::new_env_id,
    kv,
    noco::{self, ApiToken, ExistingMigrationState, MigrationState},
    url,
};

fn to_status<T: Into<anyhow::Error>>(code: StatusCode) -> impl FnOnce(T) -> ErrorResponse {
    move |err| {
        console_error!("Error: {}", err.into());
        code.into()
    }
}

pub struct AppState {
    pub kv: KvStore,
}

impl fmt::Debug for AppState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AppState").finish_non_exhaustive()
    }
}

pub fn new(state: AppState) -> Router {
    Router::new()
        .route("/bases", post(post_base))
        .with_state(Arc::new(state))
}

#[axum::debug_handler]
async fn post_base(
    State(state): State<Arc<AppState>>,
    Json(body): Json<PostBaseRequest>,
) -> Result<Json<PostBaseResponse>, ErrorResponse> {
    let env_id = new_env_id();
    let api_token = ApiToken::from(body.api_token);

    kv::put_api_token(&state.kv, &env_id, api_token.clone())
        .await
        .map_err(to_status(StatusCode::INTERNAL_SERVER_ERROR))?;

    let dash_origin =
        url::dash_origin(&body.dash_domain).map_err(to_status(StatusCode::BAD_REQUEST))?;

    let client = noco::Client::new(dash_origin.clone(), api_token);
    let migration_state = MigrationState::new(body.title, body.email);

    let ExistingMigrationState { base_id, .. } = noco::migrate(&client, migration_state)
        .await
        .map_err(to_status(StatusCode::INTERNAL_SERVER_ERROR))?;

    let dash_url = url::dash_url(dash_origin, base_id)
        .map_err(to_status(StatusCode::INTERNAL_SERVER_ERROR))?;

    let app_url = url::app_url(env_id).map_err(to_status(StatusCode::INTERNAL_SERVER_ERROR))?;

    Ok(Json(PostBaseResponse {
        dash_url: dash_url.to_string(),
        app_url: app_url.to_string(),
    }))
}
