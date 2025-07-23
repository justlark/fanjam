use std::{fmt, sync::Arc};

use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::{ErrorResponse, NoContent},
    routing::{delete, get, post, put},
};
use worker::{console_error, console_log, kv::KvStore};

use crate::{
    api::{
        Event, GetCurrentMigrationResponse, GetEventsResponse, GetLinkResponse,
        PostApplyMigrationResponse, PostBackupKind, PostBackupRequest, PostBaseRequest,
        PostLinkResponse, PostRestoreBackupKind, PostRestoreBackupRequest, PutTokenRequest,
    },
    auth::admin_auth_layer,
    cors::cors_layer,
    env::{EnvId, EnvName},
    error, kv, neon,
    noco::{
        self, ApiToken, ExistingMigrationState, MigrationState, NOCO_PRE_BASE_DELETION_BRANCH_NAME,
        NOCO_PRE_DEPLOYMENT_BRANCH_NAME, NOCO_PRE_MANUAL_RESTORE_BRANCH_NAME, check_base_exists,
    },
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
    // This service exposes two APIs: an unauthenticated "user" API for querying data that is used
    // by the client app, and an authenticated "admin" API that is used to provision and manage
    // environments.
    Router::new()
        // ADMIN API (AUTHENTICATED)
        .route("/links/{env_name}", post(post_link))
        .route("/links/{env_name}", get(get_link))
        .route("/tokens/{env_name}", put(put_token))
        .route("/bases/{env_name}", post(post_base))
        .route("/bases/{env_name}", delete(delete_base))
        .route("/migrations/{env_name}/apply", post(post_apply_migration))
        .route("/migrations/{env_name}/current", get(get_current_migration))
        .route("/backups/{env_name}", post(post_backup))
        .route("/backups/{env_name}/restore", post(post_restore_backup))
        .route_layer(admin_auth_layer())
        // USER API (UNAUTHENTICATED)
        .route("/events/{env_id}", get(get_events))
        .layer(cors_layer())
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

    let dash_url =
        url::dash_url(&env_name).map_err(to_status(StatusCode::INTERNAL_SERVER_ERROR))?;
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
        .ok_or_else(error::no_env_id)?;

    let dash_url =
        url::dash_url(&env_name).map_err(to_status(StatusCode::INTERNAL_SERVER_ERROR))?;
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
        .ok_or_else(error::no_api_token)?;

    let dash_origin =
        url::dash_origin(&env_name).map_err(to_status(StatusCode::INTERNAL_SERVER_ERROR))?;

    let noco_client = noco::Client::new(dash_origin.clone(), api_token);
    let neon_client = neon::Client::new();

    let existing_base_id = kv::get_base_id(&state.kv, &env_name)
        .await
        .map_err(to_status(StatusCode::INTERNAL_SERVER_ERROR))?;

    if let Some(base_id) = existing_base_id {
        let base_exists_in_noco = check_base_exists(&noco_client, &base_id)
            .await
            .map_err(to_status(StatusCode::INTERNAL_SERVER_ERROR))?;

        if base_exists_in_noco {
            return Err(error::base_already_exists());
        } else {
            // The NocoDB base ID was stored in KV, but the base no longer exists in NocoDB. This
            // could happen if the base is deleted manually by the system user (as opposed to via
            // this admin API), or if the environment was completely destroyed and recreated.
            console_log!(
                "The NocoDB base was deleted externally. Updating the backend state to match."
            );

            kv::delete_base_id(&state.kv, &env_name)
                .await
                .map_err(to_status(StatusCode::INTERNAL_SERVER_ERROR))?;
        }
    }

    let migration_state = MigrationState::new(body.title, body.email);

    let migrator = noco::Migrator::new(&noco_client, &neon_client, &state.kv);

    migrator
        .migrate(&env_name, migration_state)
        .await
        .map_err(to_status(StatusCode::INTERNAL_SERVER_ERROR))?;

    Ok(StatusCode::CREATED)
}

#[axum::debug_handler]
async fn delete_base(
    State(state): State<Arc<AppState>>,
    Path(env_name): Path<EnvName>,
) -> Result<NoContent, ErrorResponse> {
    let api_token = kv::get_api_token(&state.kv, &env_name)
        .await
        .map_err(to_status(StatusCode::INTERNAL_SERVER_ERROR))?
        .ok_or_else(error::no_api_token)?;

    let base_id = kv::get_base_id(&state.kv, &env_name)
        .await
        .map_err(to_status(StatusCode::INTERNAL_SERVER_ERROR))?
        .ok_or_else(error::no_base_id)?;

    let dash_origin =
        url::dash_origin(&env_name).map_err(to_status(StatusCode::INTERNAL_SERVER_ERROR))?;

    let noco_client = noco::Client::new(dash_origin.clone(), api_token);
    let neon_client = neon::Client::new();

    // Back up the database in case we delete the NocoDB base accidentally.
    neon_client
        .create_backup(
            &env_name.clone().into(),
            &NOCO_PRE_BASE_DELETION_BRANCH_NAME,
        )
        .await
        .map_err(to_status(StatusCode::INTERNAL_SERVER_ERROR))?;

    noco::delete_base(&noco_client, &base_id)
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
async fn post_apply_migration(
    State(state): State<Arc<AppState>>,
    Path(env_name): Path<EnvName>,
) -> Result<Json<PostApplyMigrationResponse>, ErrorResponse> {
    let api_token = kv::get_api_token(&state.kv, &env_name)
        .await
        .map_err(to_status(StatusCode::INTERNAL_SERVER_ERROR))?
        .ok_or_else(error::no_api_token)?;

    let base_id = kv::get_base_id(&state.kv, &env_name)
        .await
        .map_err(to_status(StatusCode::INTERNAL_SERVER_ERROR))?
        .ok_or_else(error::no_base_id)?;

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

    Ok(Json(PostApplyMigrationResponse {
        old_version,
        new_version,
    }))
}

#[axum::debug_handler]
async fn get_current_migration(
    State(state): State<Arc<AppState>>,
    Path(env_name): Path<EnvName>,
) -> Result<Json<GetCurrentMigrationResponse>, ErrorResponse> {
    let version = kv::get_migration_version(&state.kv, &env_name)
        .await
        .map_err(to_status(StatusCode::INTERNAL_SERVER_ERROR))?
        .unwrap_or(noco::Version::INITIAL);

    Ok(Json(GetCurrentMigrationResponse { version }))
}

#[axum::debug_handler]
async fn post_backup(
    Path(env_name): Path<EnvName>,
    Json(body): Json<PostBackupRequest>,
) -> Result<NoContent, ErrorResponse> {
    let neon_client = neon::Client::new();

    let dest_branch_name = match body.kind {
        PostBackupKind::Deployment => NOCO_PRE_DEPLOYMENT_BRANCH_NAME,
    };

    neon_client
        .create_backup(&env_name.clone().into(), &dest_branch_name)
        .await
        .map_err(to_status(StatusCode::INTERNAL_SERVER_ERROR))?;

    Ok(NoContent)
}

#[axum::debug_handler]
async fn post_restore_backup(
    Path(env_name): Path<EnvName>,
    Json(body): Json<PostRestoreBackupRequest>,
) -> Result<NoContent, ErrorResponse> {
    let neon_client = neon::Client::new();

    let source_branch_name = match body.kind {
        PostRestoreBackupKind::Deletion => NOCO_PRE_BASE_DELETION_BRANCH_NAME,
        PostRestoreBackupKind::Deployment => NOCO_PRE_DEPLOYMENT_BRANCH_NAME,
    };

    neon_client
        .restore_backup(
            &env_name.clone().into(),
            &source_branch_name,
            &NOCO_PRE_MANUAL_RESTORE_BRANCH_NAME,
        )
        .await
        .map_err(to_status(StatusCode::INTERNAL_SERVER_ERROR))?;

    Ok(NoContent)
}

#[axum::debug_handler]
async fn get_events(
    State(state): State<Arc<AppState>>,
    Path(env_id): Path<EnvId>,
) -> Result<Json<GetEventsResponse>, ErrorResponse> {
    let _env_name = kv::get_id_env(&state.kv, &env_id)
        .await
        .map_err(to_status(StatusCode::INTERNAL_SERVER_ERROR))?
        .ok_or(StatusCode::NOT_FOUND)?;

    //
    // TODO: Replace this dummy data with real calls to NocoDB.
    //

    let events = vec![
        Event {
            id: "1".to_string(),
            name: "Sonic the Hedgehog Cosplay Competition".to_string(),
            description: "Show off your best Sonic the Hedgehog cosplay!".to_string(),
            start_time: "2025-07-18T23:00:00Z".to_string(),
            end_time: "2025-07-19T01:00:00Z".to_string(),
            location: "Emerson Stage".to_string(),
            people: vec!["Kai".to_string(), "Alex".to_string()],
            category: "Competition".to_string(),
            tags: vec![],
        },
        Event {
            id: "2".to_string(),
            name: "Queer History Primer".to_string(),
            description: "Come learn about queer history!".to_string(),
            start_time: "2025-07-18T23:00:00Z".to_string(),
            end_time: "2025-07-19T00:30:00Z".to_string(),
            location: "Hawthorne Room".to_string(),
            people: vec!["Avery".to_string()],
            category: "Educational".to_string(),
            tags: vec!["LGBTQ".to_string(), "Q&A".to_string()],
        },
        Event {
            id: "3".to_string(),
            name: "The Unexpected Comforts of Living in the Woods".to_string(),
            description: "Living in the woods is hard. You're far away from everything, you have critters living in your walls, and the housework keeps piling up. I love it. Let me share with you the unexpected comforts I found living in the woods.".to_string(),
            start_time: "2025-07-19T14:00:00Z".to_string(),
            end_time: "2025-07-19T15:30:00Z".to_string(),
            location: "Thoreau Room".to_string(),
            people: vec!["Ash".to_string()],
            category: "Educational".to_string(),
            tags: vec!["Q&A".to_string()],
        },
        Event {
            id: "4".to_string(),
            name: "Chainmaille 101".to_string(),
            description: "In this class, you'll make a simple pair of chainmaille earrings. There is a small fee to cover the cost of supplies.".to_string(),
            start_time: "2025-07-19T15:30:00Z".to_string(),
            end_time: "2025-07-19T17:00:00Z".to_string(),
            location: "Alcott Room".to_string(),
            people: vec!["Blue".to_string()],
            category: "Class".to_string(),
            tags: vec!["$$$".to_string()],
        },
        Event {
            id: "5".to_string(),
            name: "The Future".to_string(),
            description: "This class takes place a long time from now.".to_string(),
            start_time: "2026-02-02T18:00:00Z".to_string(),
            end_time: "2026-02-02T19:00:00Z".to_string(),
            location: "Space".to_string(),
            people: vec!["Zephyr".to_string()],
            category: "Experience".to_string(),
            tags: vec!["Cool".to_string()],
        },
    ];

    Ok(Json(GetEventsResponse { events }))
}
