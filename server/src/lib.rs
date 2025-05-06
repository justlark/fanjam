mod api;
mod config;
mod noco;
mod router;

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

    Ok(router::new().call(req).await?)
}
