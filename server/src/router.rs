use std::{fmt, sync::Arc};

use axum::{
    Json, Router,
    extract::{Path, State},
    response::{ErrorResponse, NoContent},
    routing::{get, post, put},
};
use reqwest::StatusCode;
use worker::{console_error, kv::KvStore};

use crate::{
    api::{GetLinkResponse, PostBaseRequest, PostLinkResponse, PutTokenRequest},
    auth::admin_auth_layer,
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
        // AUTHENTICATED ENDPOINTS
        .route("/links/{env_name}", post(post_link))
        .route("/links/{env_name}", get(get_link))
        .route("/tokens/{env_name}", put(put_token))
        .route("/bases/{env_name}", post(post_base))
        .route_layer(admin_auth_layer())
        // UNAUTHENTICATED ENDPOINTS
        .with_state(Arc::new(state))
}

#[axum::debug_handler]
async fn post_link(
    State(state): State<Arc<AppState>>,
    Path(env_name): Path<String>,
) -> Result<Json<PostLinkResponse>, ErrorResponse> {
    let dash_origin = url::dash_origin(&env_name).map_err(to_status(StatusCode::BAD_REQUEST))?;

    let env_id = kv::put_env_id(&state.kv, &env_name)
        .await
        .map_err(to_status(StatusCode::INTERNAL_SERVER_ERROR))?;

    let dash_url =
        url::dash_url(dash_origin).map_err(to_status(StatusCode::INTERNAL_SERVER_ERROR))?;

    let app_url = url::app_url(&env_id).map_err(to_status(StatusCode::INTERNAL_SERVER_ERROR))?;

    Ok(Json(PostLinkResponse {
        dash_url: dash_url.to_string(),
        app_url: app_url.to_string(),
    }))
}

#[axum::debug_handler]
async fn get_link(
    State(state): State<Arc<AppState>>,
    Path(env_name): Path<String>,
) -> Result<Json<GetLinkResponse>, ErrorResponse> {
    let env_id = kv::get_env_id(&state.kv, &env_name)
        .await
        .map_err(to_status(StatusCode::INTERNAL_SERVER_ERROR))?
        .ok_or(ErrorResponse::from(StatusCode::NOT_FOUND))?;

    let dash_origin =
        url::dash_origin(&env_name).map_err(to_status(StatusCode::INTERNAL_SERVER_ERROR))?;

    let dash_url =
        url::dash_url(dash_origin).map_err(to_status(StatusCode::INTERNAL_SERVER_ERROR))?;

    let app_url = url::app_url(&env_id).map_err(to_status(StatusCode::INTERNAL_SERVER_ERROR))?;

    Ok(Json(GetLinkResponse {
        dash_url: dash_url.to_string(),
        app_url: app_url.to_string(),
    }))
}

#[axum::debug_handler]
async fn put_token(
    State(state): State<Arc<AppState>>,
    Path(env_name): Path<String>,
    Json(body): Json<PutTokenRequest>,
) -> Result<NoContent, ErrorResponse> {
    kv::put_api_token(&state.kv, &env_name, ApiToken::from(body.token))
        .await
        .map_err(to_status(StatusCode::INTERNAL_SERVER_ERROR))?;

    Ok(NoContent)
}

#[axum::debug_handler]
async fn post_base(
    State(state): State<Arc<AppState>>,
    Path(env_name): Path<String>,
    Json(body): Json<PostBaseRequest>,
) -> Result<StatusCode, ErrorResponse> {
    let api_token = kv::get_api_token(&state.kv, &env_name)
        .await
        .map_err(to_status(StatusCode::INTERNAL_SERVER_ERROR))?
        .ok_or(ErrorResponse::from(StatusCode::NOT_FOUND))?;

    let dash_origin =
        url::dash_origin(&env_name).map_err(to_status(StatusCode::INTERNAL_SERVER_ERROR))?;

    let client = noco::Client::new(dash_origin.clone(), api_token);
    let migration_state = MigrationState::new(body.title, body.email);

    let ExistingMigrationState { base_id, .. } = noco::migrate(&client, migration_state)
        .await
        .map_err(to_status(StatusCode::INTERNAL_SERVER_ERROR))?;

    kv::put_base_id(&state.kv, &env_name, &base_id)
        .await
        .map_err(to_status(StatusCode::INTERNAL_SERVER_ERROR))?;

    Ok(StatusCode::CREATED)
}
