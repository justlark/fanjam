use axum::http::{
    Method,
    header::{CONTENT_TYPE, ETAG, IF_NONE_MATCH},
};
use tower_http::cors::{Any, CorsLayer};

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
        .allow_origin(Any)
}
