use std::time::Duration;

use axum::http::{
    Method,
    header::{CONTENT_TYPE, ETAG, IF_NONE_MATCH},
};
use tower_http::cors::{Any, CorsLayer};

// It is unlikely that the CORS headers will change frequently, so we can avoid unnecessary
// preflight requests.
const PREFLIGHT_MAX_AGE: Duration = Duration::from_secs(60 * 60);

pub fn cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
        ])
        .allow_headers([CONTENT_TYPE, IF_NONE_MATCH])
        .expose_headers([ETAG])
        .max_age(PREFLIGHT_MAX_AGE)
        .allow_origin(Any)
}
