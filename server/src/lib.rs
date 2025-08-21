mod api;
mod auth;
mod cache;
mod config;
mod cors;
mod env;
mod error;
mod http;
mod kv;
mod neon;
mod noco;
mod router;
mod sql;
mod store;
mod upstash;
mod url;

use router::AppState;
use tower_service::Service;
use worker::*;

#[event(fetch)]
async fn fetch(
    req: HttpRequest,
    env: Env,
    _ctx: Context,
) -> Result<axum::http::Response<axum::body::Body>> {
    console_error_panic_hook::set_once();

    config::init(&env).expect("failed to initialize config");

    let state = AppState { kv: env.kv("KV")? };

    Ok(router::new(state).call(req).await?)
}
