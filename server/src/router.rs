use std::{fmt, sync::Arc};

use axum::{
    Json, Router,
    extract::{Path, State},
    response::{ErrorResponse, NoContent},
    routing::{delete, get, post, put},
};
use reqwest::StatusCode;
use worker::{console_error, kv::KvStore};

use crate::{
    api::{
        GetLinkResponse, GetMigrationResponse, PostBaseRequest, PostLinkResponse,
        PostMigrationResponse, PutTokenRequest,
    },
    auth::admin_auth_layer,
    env::{EnvId, EnvName},
    error::{err_base_already_exists, err_no_api_token, err_no_base_id, err_no_env_id},
    kv, neon,
    noco::{self, ApiToken, ExistingMigrationState, MigrationState},
    url,
};

//
// There are going to be race conditions inherent to this implementation. Because we have our own
// state that we're trying to keep in sync with the state of NocoDB, there is no easy way around
// it. We're not worrying too much about possible race condition here because:
//
// 1. It really just applies to the internal admin API, not the client-facing API. These internal
//    admin API endpoints are not going to be used frequently or by many different people
//    concurrently.
// 2. Any inconsistencies that arise, such as from operations failing at critical points, can
//    probably be resolved manually. These internal admin API endpoints are intended to be called
//    manually, and functionality is split out into distinct endpoints to provide granular control
//    over the state of environments.
//

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
        .route("/bases/{env_name}", delete(delete_base))
        .route("/migrations/{env_name}", post(post_migration))
        .route("/migrations/{env_name}", get(get_migration))
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
        .ok_or_else(err_no_env_id)?;

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
        .ok_or_else(err_no_api_token)?;

    let existing_base_id = kv::get_base_id(&state.kv, &env_name)
        .await
        .map_err(to_status(StatusCode::INTERNAL_SERVER_ERROR))?;

    if existing_base_id.is_some() {
        return Err(err_base_already_exists());
    }

    let dash_origin =
        url::dash_origin(&env_name).map_err(to_status(StatusCode::INTERNAL_SERVER_ERROR))?;

    let noco_client = noco::Client::new(dash_origin.clone(), api_token);
    let neon_client = neon::Client::new();

    let migration_state = MigrationState::new(body.title, body.email);

    let migrator = noco::Migrator::new(&noco_client, &neon_client, &state.kv);

    migrator
        .migrate(&env_name, migration_state)
        .await
        .map_err(to_status(StatusCode::INTERNAL_SERVER_ERROR))?;

    Ok(StatusCode::CREATED)
}

// TODO: Branch the Neon database before deleting the base.
#[axum::debug_handler]
async fn delete_base(
    State(state): State<Arc<AppState>>,
    Path(env_name): Path<EnvName>,
) -> Result<NoContent, ErrorResponse> {
    let api_token = kv::get_api_token(&state.kv, &env_name)
        .await
        .map_err(to_status(StatusCode::INTERNAL_SERVER_ERROR))?
        .ok_or_else(err_no_api_token)?;

    let base_id = kv::get_base_id(&state.kv, &env_name)
        .await
        .map_err(to_status(StatusCode::INTERNAL_SERVER_ERROR))?
        .ok_or_else(err_no_base_id)?;

    let dash_origin =
        url::dash_origin(&env_name).map_err(to_status(StatusCode::INTERNAL_SERVER_ERROR))?;

    let client = noco::Client::new(dash_origin.clone(), api_token);

    noco::delete_base(&client, &base_id)
        .await
        .map_err(to_status(StatusCode::INTERNAL_SERVER_ERROR))?;

    kv::delete_base_id(&state.kv, &env_name)
        .await
        .map_err(to_status(StatusCode::INTERNAL_SERVER_ERROR))?;

    kv::delete_migration_version(&state.kv, &env_name)
        .await
        .map_err(to_status(StatusCode::INTERNAL_SERVER_ERROR))?;

    Ok(NoContent)
}

#[axum::debug_handler]
async fn post_migration(
    State(state): State<Arc<AppState>>,
    Path(env_name): Path<EnvName>,
) -> Result<Json<PostMigrationResponse>, ErrorResponse> {
    let api_token = kv::get_api_token(&state.kv, &env_name)
        .await
        .map_err(to_status(StatusCode::INTERNAL_SERVER_ERROR))?
        .ok_or_else(err_no_api_token)?;

    let base_id = kv::get_base_id(&state.kv, &env_name)
        .await
        .map_err(to_status(StatusCode::INTERNAL_SERVER_ERROR))?
        .ok_or_else(err_no_base_id)?;

    let old_version = kv::get_migration_version(&state.kv, &env_name)
        .await
        .map_err(to_status(StatusCode::INTERNAL_SERVER_ERROR))?
        .unwrap_or(noco::Version::INITIAL);

    let dash_origin =
        url::dash_origin(&env_name).map_err(to_status(StatusCode::INTERNAL_SERVER_ERROR))?;

    let noco_client = noco::Client::new(dash_origin.clone(), api_token);
    let neon_client = neon::Client::new();

    let migration_state = MigrationState::existing(old_version, base_id);

    let migrator = noco::Migrator::new(&noco_client, &neon_client, &state.kv);

    let ExistingMigrationState {
        version: new_version,
        ..
    } = migrator
        .migrate(&env_name, migration_state)
        .await
        .map_err(to_status(StatusCode::INTERNAL_SERVER_ERROR))?;

    Ok(Json(PostMigrationResponse {
        old_version,
        new_version,
    }))
}

#[axum::debug_handler]
async fn get_migration(
    State(state): State<Arc<AppState>>,
    Path(env_name): Path<EnvName>,
) -> Result<Json<GetMigrationResponse>, ErrorResponse> {
    let version = kv::get_migration_version(&state.kv, &env_name)
        .await
        .map_err(to_status(StatusCode::INTERNAL_SERVER_ERROR))?
        .unwrap_or(noco::Version::INITIAL);

    Ok(Json(GetMigrationResponse { version }))
}
