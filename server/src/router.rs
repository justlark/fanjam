use std::{fmt, sync::Arc};

use axum::{
    Json, Router,
    body::Body,
    extract::{Path, State},
    http::{self, StatusCode, Uri},
    middleware,
    response::{ErrorResponse, IntoResponse, NoContent},
    routing::{delete, get, post, put},
};
use worker::{Bucket, Cache, Context, console_log, kv::KvStore, send::SendWrapper};

use crate::{
    api::{
        Announcement, DataResponseEnvelope, Event, File, GetAnnouncementsResponse,
        GetConfigResponse, GetCurrentMigrationResponse, GetEventsResponse, GetInfoResponse,
        GetLinkResponse, GetPagesResponse, GetSummaryResponse, Link, Page,
        PostApplyMigrationResponse, PostBackupRequest, PostBaseRequest, PostLinkResponse,
        PostRestoreBackupKind, PostRestoreBackupRequest, PutTokenRequest,
    },
    auth::admin_auth_layer,
    cache::{EtagJson, get_cdn_cache, if_none_match_middleware, put_cdn_cache},
    cf, config,
    cors::cors_layer,
    env::{CONFIG_DOCS, Config, EnvId, EnvName},
    error::Error,
    http::http_headers_from_object,
    kv, neon,
    noco::{self, ApiToken, MigrationState},
    sql,
    store::{self, MigrationChange, Store},
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

pub struct AppState {
    pub kv: KvStore,
    pub bucket: SendWrapper<Bucket>,
    pub ctx: Arc<Context>,
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
        .route("/admin/env/{env_name}/links", post(post_link))
        .route("/admin/env/{env_name}/links", get(get_link))
        .route("/admin/env/{env_name}/tokens", put(put_token))
        .route("/admin/env/{env_name}/bases", post(post_base))
        .route("/admin/env/{env_name}/bases", delete(delete_base))
        .route(
            "/admin/env/{env_name}/migrations/apply",
            post(post_apply_migration),
        )
        .route(
            "/admin/env/{env_name}/migrations/current",
            get(get_current_migration),
        )
        .route("/admin/env/{env_name}/backups", post(post_backup))
        .route(
            "/admin/env/{env_name}/backups/restore",
            post(post_restore_backup),
        )
        .route("/admin/env/{env_name}/cache", delete(delete_cache))
        .route("/admin/env/{env_name}/config", get(get_admin_config))
        .route("/admin/env/{env_name}/config", put(put_admin_config))
        .route("/admin/config-docs", get(get_config_docs))
        .route_layer(admin_auth_layer())
        // USER API (UNAUTHENTICATED)
        .route("/apps/{env_id}/events", get(get_events))
        .route("/apps/{env_id}/info", get(get_info))
        .route("/apps/{env_id}/pages", get(get_pages))
        .route("/apps/{env_id}/announcements", get(get_announcements))
        .route("/apps/{env_id}/summary", get(get_summary))
        .route("/apps/{env_id}/config", get(get_config))
        .route("/apps/{env_id}/assets/{name}", get(get_asset))
        .layer(middleware::from_fn(if_none_match_middleware))
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
        .map_err(Error::Internal)?;

    kv::put_env_id(&state.kv, &env_name, &env_id)
        .await
        .map_err(Error::Internal)?;

    let dash_url = url::dash_url(&env_name).map_err(Error::Internal)?;
    let app_url = url::app_url(&env_id).map_err(Error::Internal)?;

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
        .map_err(Error::Internal)?
        .ok_or(Error::NoEnvId)?;

    let dash_url = url::dash_url(&env_name).map_err(Error::Internal)?;
    let app_url = url::app_url(&env_id).map_err(Error::Internal)?;
    let local_url = url::local_url(&env_id).map_err(Error::Internal)?;

    Ok(Json(GetLinkResponse {
        dash_url: dash_url.to_string(),
        app_url: app_url.to_string(),
        local_url: local_url.to_string(),
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
        .map_err(Error::Internal)?;

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
        .map_err(Error::Internal)?
        .ok_or(Error::NoApiToken)?;

    let dash_origin = url::dash_origin(&env_name).map_err(Error::Internal)?;

    let env_config = kv::get_env_config(&state.kv, &env_name)
        .await
        .map_err(Error::Internal)?;

    let noco_client = noco::Client::new(dash_origin.clone(), api_token);
    let neon_client = neon::Client::new();
    let db_client = sql::Client::connect(
        &Option::<sql::ConnectionConfig>::from(env_config).ok_or(Error::MissingEnvConfig)?,
    )
    .await
    .map_err(Error::Internal)?;

    let existing_base_id = db_client.get_base().await.map_err(Error::Internal)?;

    if let Some(base_id) = existing_base_id {
        let base_exists_in_noco = noco::check_base_exists(&noco_client, &base_id)
            .await
            .map_err(Error::Internal)?;

        if base_exists_in_noco {
            Err(Error::BaseAlreadyExists)?;
        } else {
            // The NocoDB base ID was stored in the config database, but the base no longer exists
            // in NocoDB. This could happen if the base is deleted manually by the system user (as
            // opposed to via this admin API), or if the environment was completely destroyed and
            // recreated.
            console_log!(
                "The NocoDB base was deleted externally. Updating the backend state to match."
            );

            db_client
                .delete_base(&base_id)
                .await
                .map_err(Error::Internal)?;
        }
    }

    let migration_state = MigrationState::new(body.title, body.email);

    let migrator = noco::Migrator::new(&noco_client, &neon_client, &db_client);

    migrator
        .migrate(&env_name, migration_state)
        .await
        .map_err(Error::Internal)?;

    Ok(StatusCode::CREATED)
}

#[axum::debug_handler]
async fn delete_base(
    State(state): State<Arc<AppState>>,
    Path(env_name): Path<EnvName>,
) -> Result<NoContent, ErrorResponse> {
    let store = Store::from_env_name(&state, env_name).await?;

    store.delete_base().await?;

    Ok(NoContent)
}

#[axum::debug_handler]
async fn post_apply_migration(
    State(state): State<Arc<AppState>>,
    Path(env_name): Path<EnvName>,
) -> Result<Json<PostApplyMigrationResponse>, ErrorResponse> {
    let store = Store::from_env_name(&state, env_name).await?;

    let MigrationChange {
        old_version,
        new_version,
    } = store.migrate().await?;

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
    let env_config = kv::get_env_config(&state.kv, &env_name)
        .await
        .map_err(Error::Internal)?;

    let db_client = sql::Client::connect(
        &Option::<sql::ConnectionConfig>::from(env_config).ok_or(Error::MissingEnvConfig)?,
    )
    .await
    .map_err(Error::Internal)?;

    let version = db_client
        .get_current_migration()
        .await
        .map_err(Error::Internal)?;

    Ok(Json(GetCurrentMigrationResponse { version }))
}

#[axum::debug_handler]
async fn post_backup(
    State(state): State<Arc<AppState>>,
    Path(env_name): Path<EnvName>,
    Json(body): Json<PostBackupRequest>,
) -> Result<NoContent, ErrorResponse> {
    let store = Store::from_env_name(&state, env_name).await?;

    store.create_backup(body.kind).await?;

    Ok(NoContent)
}

#[axum::debug_handler]
async fn post_restore_backup(
    Path(env_name): Path<EnvName>,
    Json(body): Json<PostRestoreBackupRequest>,
) -> Result<NoContent, ErrorResponse> {
    let backup_kind = match body.kind {
        PostRestoreBackupKind::Deletion => neon::BackupKind::Deletion,
        PostRestoreBackupKind::Deployment => neon::BackupKind::Deployment,
        PostRestoreBackupKind::Migration => neon::BackupKind::Migration,
    };

    let neon_client = neon::Client::new();
    // let upstash_client = upstash::Client::new();

    // Since we're rolling back the database, we should clear the Redis cache as well so the
    // client doesn't get confused.
    // upstash_client
    //     .unlink_noco_keys(&env_name)
    //     .await
    //     .map_err(Error::Internal)?;

    neon_client
        .restore_backup(&env_name.clone().into(), backup_kind)
        .await
        .map_err(Error::Internal)?;

    Ok(NoContent)
}

#[axum::debug_handler]
#[worker::send]
async fn delete_cache(
    State(state): State<Arc<AppState>>,
    Path(env_name): Path<EnvName>,
) -> Result<NoContent, ErrorResponse> {
    kv::delete_cache(&state.kv, &env_name)
        .await
        .map_err(Error::Internal)?;

    let cloudflare_client = cf::Client::new();

    cloudflare_client
        .purge_cache(
            &config::cloudflare_zone_id(),
            &cf::CacheTag::for_env(&env_name),
        )
        .await
        .map_err(Error::Internal)?;

    // let upstash_client = upstash::Client::new();

    // upstash_client
    //     .unlink_noco_keys(&env_name)
    //     .await
    //     .map_err(Error::Internal)?;

    Ok(NoContent)
}

#[axum::debug_handler]
async fn get_admin_config(
    State(state): State<Arc<AppState>>,
    Path(env_name): Path<EnvName>,
) -> Result<Json<Config>, ErrorResponse> {
    Ok(Json(
        kv::get_env_config(&state.kv, &env_name)
            .await
            .map_err(Error::Internal)?,
    ))
}

#[axum::debug_handler]
async fn put_admin_config(
    State(state): State<Arc<AppState>>,
    Path(env_name): Path<EnvName>,
    Json(config): Json<Config>,
) -> Result<NoContent, ErrorResponse> {
    kv::put_env_config(&state.kv, &env_name, &config)
        .await
        .map_err(Error::Internal)?;

    Ok(NoContent)
}

#[axum::debug_handler]
async fn get_config_docs() -> Result<Json<serde_json::Value>, ErrorResponse> {
    Ok(serde_json::from_str::<serde_json::Value>(CONFIG_DOCS)
        .map(Json)
        .map_err(anyhow::Error::from)
        .map_err(Error::Internal)?)
}

#[axum::debug_handler]
#[worker::send]
async fn get_events(
    State(state): State<Arc<AppState>>,
    uri: Uri,
    Path(env_id): Path<EnvId>,
) -> Result<http::Response<Body>, ErrorResponse> {
    let cache = Cache::default();

    if let Some(response) = get_cdn_cache(&cache, uri.clone()).await? {
        return Ok(response);
    };

    let store = Store::from_env_id(&state, &env_id).await?;

    let store::DataResponseEnvelope {
        retry_after,
        value: events,
    } = store.get_events().await?;

    let response = EtagJson(DataResponseEnvelope {
        retry_after_ms: retry_after.map(|d| d.as_millis() as u64),
        value: GetEventsResponse {
            events: events
                .into_iter()
                .map(|event| Event {
                    id: event.id,
                    name: event.name,
                    summary: event.summary,
                    description: event.description,
                    start_time: event.start_time,
                    end_time: event.end_time,
                    location: event.location,
                    people: event.people,
                    category: event.category,
                    tags: event.tags,
                })
                .collect::<Vec<_>>(),
        },
    })
    .into_response();

    let response = put_cdn_cache(
        &state.ctx,
        cache,
        store.env_name().to_owned(),
        store.cache_ttl(),
        uri,
        response,
    )?;

    Ok(response)
}

#[axum::debug_handler]
#[worker::send]
async fn get_info(
    State(state): State<Arc<AppState>>,
    uri: Uri,
    Path(env_id): Path<EnvId>,
) -> Result<http::Response<Body>, ErrorResponse> {
    let cache = Cache::default();

    if let Some(response) = get_cdn_cache(&cache, uri.clone()).await? {
        return Ok(response);
    };

    let store = Store::from_env_id(&state, &env_id).await?;

    let store::DataResponseEnvelope {
        retry_after,
        value: info,
    } = store.get_info().await?;

    let response = EtagJson(DataResponseEnvelope {
        retry_after_ms: retry_after.map(|d| d.as_millis() as u64),
        value: GetInfoResponse {
            name: info.about.name.clone(),
            description: info.about.description.clone(),
            website_url: info.about.website_url.clone(),
            links: info
                .links
                .into_iter()
                .map(|link| Link {
                    name: link.name,
                    url: link.url,
                })
                .collect::<Vec<_>>(),
            files: info
                .about
                .files
                .into_iter()
                .map(|file| File {
                    name: file.name,
                    media_type: file.media_type,
                    signed_url: file.signed_url,
                })
                .collect::<Vec<_>>(),
        },
    })
    .into_response();

    let response = put_cdn_cache(
        &state.ctx,
        cache,
        store.env_name().to_owned(),
        store.cache_ttl(),
        uri,
        response,
    )?;

    Ok(response)
}

#[axum::debug_handler]
#[worker::send]
async fn get_pages(
    State(state): State<Arc<AppState>>,
    uri: Uri,
    Path(env_id): Path<EnvId>,
) -> Result<http::Response<Body>, ErrorResponse> {
    let cache = Cache::default();

    if let Some(response) = get_cdn_cache(&cache, uri.clone()).await? {
        return Ok(response);
    }

    let store = Store::from_env_id(&state, &env_id).await?;

    let store::DataResponseEnvelope {
        retry_after,
        value: pages,
    } = store.get_pages().await?;

    let response = EtagJson(DataResponseEnvelope {
        retry_after_ms: retry_after.map(|d| d.as_millis() as u64),
        value: GetPagesResponse {
            pages: pages
                .into_iter()
                .map(|page| Page {
                    id: page.id,
                    title: page.title,
                    body: page.body,
                    files: page
                        .files
                        .into_iter()
                        .map(|file| File {
                            name: file.name,
                            media_type: file.media_type,
                            signed_url: file.signed_url,
                        })
                        .collect::<Vec<_>>(),
                })
                .collect::<Vec<_>>(),
        },
    })
    .into_response();

    let response = put_cdn_cache(
        &state.ctx,
        cache,
        store.env_name().to_owned(),
        store.cache_ttl(),
        uri,
        response,
    )?;

    Ok(response)
}

#[axum::debug_handler]
#[worker::send]
async fn get_announcements(
    State(state): State<Arc<AppState>>,
    uri: Uri,
    Path(env_id): Path<EnvId>,
) -> Result<http::Response<Body>, ErrorResponse> {
    let cache = Cache::default();

    if let Some(response) = get_cdn_cache(&cache, uri.clone()).await? {
        return Ok(response);
    }

    let store = Store::from_env_id(&state, &env_id).await?;

    let store::DataResponseEnvelope {
        retry_after,
        value: announcements,
    } = store.get_announcements().await?;

    let response = EtagJson(DataResponseEnvelope {
        retry_after_ms: retry_after.map(|d| d.as_millis() as u64),
        value: GetAnnouncementsResponse {
            announcements: announcements
                .into_iter()
                .map(|announcement| Announcement {
                    id: announcement.id,
                    title: announcement.title,
                    body: announcement.body,
                    attachments: announcement
                        .files
                        .into_iter()
                        .map(|file| File {
                            name: file.name,
                            media_type: file.media_type,
                            signed_url: file.signed_url,
                        })
                        .collect::<Vec<_>>(),
                    created_at: announcement.created_at,
                    updated_at: announcement.updated_at,
                })
                .collect::<Vec<_>>(),
        },
    })
    .into_response();

    let response = put_cdn_cache(
        &state.ctx,
        cache,
        store.env_name().to_owned(),
        store.cache_ttl(),
        uri,
        response,
    )?;

    Ok(response)
}

#[axum::debug_handler]
#[worker::send]
async fn get_summary(
    State(state): State<Arc<AppState>>,
    uri: Uri,
    Path(env_id): Path<EnvId>,
) -> Result<http::Response<Body>, ErrorResponse> {
    let cache = Cache::default();

    if let Some(response) = get_cdn_cache(&cache, uri.clone()).await? {
        return Ok(response);
    }

    let store = Store::from_env_id(&state, &env_id).await?;

    let summary = store.get_summary().await?;

    let response = Json(GetSummaryResponse {
        env_name: store.env_name().to_string(),
        name: summary.name,
        description: summary.description,
    })
    .into_response();

    let response = put_cdn_cache(
        &state.ctx,
        cache,
        store.env_name().to_owned(),
        config::noco_summary_cache_ttl(),
        uri,
        response,
    )?;

    Ok(response)
}

#[axum::debug_handler]
async fn get_config(
    State(state): State<Arc<AppState>>,
    Path(env_id): Path<EnvId>,
) -> Result<Json<GetConfigResponse>, ErrorResponse> {
    let env_name = kv::get_id_env(&state.kv, &env_id)
        .await
        .map_err(Error::Internal)?
        .ok_or(Error::NoEnvId)?;

    let config = kv::get_env_config(&state.kv, &env_name)
        .await
        .map_err(Error::Internal)?;

    Ok(Json(GetConfigResponse {
        timezone: config.timezone,
        use_custom_icon: config.use_custom_icon,
        favicon_name: config.favicon_name,
        opengraph_icon_name: config.opengraph_icon_name,
        opengraph_icon_type: config.opengraph_icon_type,
        opengraph_icon_alt: config.opengraph_icon_alt,
        pwa_short_app_name: config.pwa_short_app_name,
        pwa_background_color: config.pwa_background_color,
        pwa_icon_any_name: config.pwa_icon_any_name,
        pwa_icon_any_type: config.pwa_icon_any_type,
        pwa_icon_any_sizes: config.pwa_icon_any_sizes,
        pwa_icon_maskable_name: config.pwa_icon_maskable_name,
        pwa_icon_maskable_type: config.pwa_icon_maskable_type,
        pwa_icon_maskable_sizes: config.pwa_icon_maskable_sizes,
        pwa_icon_monochrome_name: config.pwa_icon_monochrome_name,
        pwa_icon_monochrome_type: config.pwa_icon_monochrome_type,
        pwa_icon_monochrome_sizes: config.pwa_icon_monochrome_sizes,
        pwa_icon_monochrome_maskable_name: config.pwa_icon_monochrome_maskable_name,
        pwa_icon_monochrome_maskable_type: config.pwa_icon_monochrome_maskable_type,
        pwa_icon_monochrome_maskable_sizes: config.pwa_icon_monochrome_maskable_sizes,
    }))
}

#[axum::debug_handler]
#[worker::send]
async fn get_asset(
    State(state): State<Arc<AppState>>,
    uri: Uri,
    Path((env_id, name)): Path<(EnvId, String)>,
) -> Result<http::Response<Body>, ErrorResponse> {
    let cache = Cache::default();

    if let Some(cached_response) = get_cdn_cache(&cache, uri.clone()).await? {
        return Ok(cached_response);
    };

    let env_name = kv::get_id_env(&state.kv, &env_id)
        .await
        .map_err(Error::Internal)?
        .ok_or(Error::NoEnvId)?;

    let bucket_key = format!("env/{env_name}/{name}");

    let object = state
        .bucket
        .get(&bucket_key)
        .execute()
        .await
        .map_err(|err| Error::Internal(err.into()))?
        .ok_or(Error::AssetNotFound)?;

    let response_headers = http_headers_from_object(&object).map_err(Error::Internal)?;

    response_headers
        .set("Cache-Control", "public, no-cache")
        .map_err(anyhow::Error::from)
        .map_err(Error::Internal)?;

    // Using `ObjectBody::response_body()` here is important because it offloads streaming the data
    // to the Workers runtime, which saves us CPU time (and therefore money).
    let response_body = object
        .body()
        .ok_or(Error::AssetNotFound)?
        .response_body()
        .map_err(|err| Error::Internal(err.into()))?;

    let response = http::Response::from(
        worker::Response::from_body(response_body)
            .map_err(anyhow::Error::from)
            .map_err(Error::Internal)?
            .with_headers(response_headers),
    );

    let response = put_cdn_cache(
        &state.ctx,
        cache,
        env_name,
        config::r2_asset_cache_ttl(),
        uri,
        response,
    )?;

    Ok(response)
}
