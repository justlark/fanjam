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
    api::{
        ErrorResponse as ApiErrorResponse, GetLinkResponse, PostBaseRequest, PostLinkResponse,
        PutTokenRequest,
    },
    auth::admin_auth_layer,
    env::{EnvId, EnvName},
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

fn error_response(code: StatusCode, message: &str) -> ErrorResponse {
    console_error!("Error: {}", message);
    ErrorResponse::from((
        code,
        Json(ApiErrorResponse {
            error: message.to_string(),
        }),
    ))
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
    Path(env_name): Path<EnvName>,
) -> Result<Json<PostLinkResponse>, ErrorResponse> {
    let env_id = EnvId::new();

    kv::put_id_env(&state.kv, &env_id, &env_name)
        .await
        .map_err(to_status(StatusCode::INTERNAL_SERVER_ERROR))?;

    kv::put_env_id(&state.kv, &env_name, &env_id)
        .await
        .map_err(to_status(StatusCode::INTERNAL_SERVER_ERROR))?;

    let dash_origin =
        url::dash_origin(&env_name).map_err(to_status(StatusCode::INTERNAL_SERVER_ERROR))?;
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
    Path(env_name): Path<EnvName>,
) -> Result<Json<GetLinkResponse>, ErrorResponse> {
    let env_id = kv::get_env_id(&state.kv, &env_name)
        .await
        .map_err(to_status(StatusCode::INTERNAL_SERVER_ERROR))?
        .ok_or_else(|| {
            error_response(
                StatusCode::NOT_FOUND,
                "You have not generated an app link for this environment.",
            )
        })?;

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
    Path(env_name): Path<EnvName>,
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
    Path(env_name): Path<EnvName>,
    Json(body): Json<PostBaseRequest>,
) -> Result<StatusCode, ErrorResponse> {
    let api_token = kv::get_api_token(&state.kv, &env_name)
        .await
        .map_err(to_status(StatusCode::INTERNAL_SERVER_ERROR))?
        .ok_or_else(|| {
            error_response(
                StatusCode::CONFLICT,
                "There is no NocoDB API token configured for this environment.",
            )
        })?;

    let existing_base_id = kv::get_base_id(&state.kv, &env_name)
        .await
        .map_err(to_status(StatusCode::INTERNAL_SERVER_ERROR))?;

    // There is a race condition here. NocoDB probably does not provide a way to perform this check
    // atomically. However, in practice this is probably Good Enough.
    if existing_base_id.is_some() {
        return Err(error_response(
            StatusCode::CONFLICT,
            "A NocoDB base already exists for this environment.",
        ));
    }

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
