use axum::http::{
    HeaderName, Method,
    header::{AUTHORIZATION, CONTENT_TYPE},
};
use tower_http::cors::CorsLayer;

use crate::config;

const LOCAL_HOST: &str = "http://localhost:5173";

const CORS_ALLOWED_METHODS: [Method; 5] = [
    Method::GET,
    Method::POST,
    Method::PUT,
    Method::PATCH,
    Method::DELETE,
];

const CORS_ALLOWED_HEADERS: [HeaderName; 2] = [CONTENT_TYPE, AUTHORIZATION];

pub fn cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_methods(CORS_ALLOWED_METHODS)
        .allow_headers(CORS_ALLOWED_HEADERS)
        .allow_origin([
            format!("https://{}", config::client_domain())
                .parse()
                .unwrap(),
            LOCAL_HOST.parse().unwrap(),
        ])
}
