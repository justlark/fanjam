use anyhow::Context;
use axum::{
    body::Body,
    http::{Request, Response, StatusCode, header::AUTHORIZATION},
    response::IntoResponse,
};
use base64::prelude::*;
use constant_time_eq::constant_time_eq;
use futures::future::{BoxFuture, FutureExt};
use secrecy::{ExposeSecret, SecretSlice};
use tower_http::auth::AsyncRequireAuthorizationLayer;

use crate::config;

const BEARER_PREFIX: &str = "Bearer ";

#[derive(Debug, Clone)]
pub struct ApiToken(SecretSlice<u8>);

impl From<Vec<u8>> for ApiToken {
    fn from(bytes: Vec<u8>) -> Self {
        Self(SecretSlice::from(bytes))
    }
}

impl TryFrom<&str> for ApiToken {
    type Error = anyhow::Error;

    fn try_from(token: &str) -> Result<Self, Self::Error> {
        let bytes = BASE64_STANDARD
            .decode(token)
            .context("failed to decode admin API token")?;
        Ok(Self::from(bytes))
    }
}

type BoxFutureResponseResult<'a> = BoxFuture<'a, Result<Request<Body>, Response<Body>>>;

pub fn admin_auth_layer<'a>()
-> AsyncRequireAuthorizationLayer<impl Fn(Request<Body>) -> BoxFutureResponseResult<'a> + Clone> {
    AsyncRequireAuthorizationLayer::new(|req: Request<Body>| {
        async move {
            let auth_header = req
                .headers()
                .get(AUTHORIZATION)
                .ok_or_else(|| StatusCode::UNAUTHORIZED.into_response())?;

            let auth_header_value = auth_header
                .to_str()
                .map_err(|_| StatusCode::UNAUTHORIZED.into_response())?;

            let actual_api_token = auth_header_value
                .strip_prefix(BEARER_PREFIX)
                .map(ApiToken::try_from)
                .ok_or_else(|| StatusCode::UNAUTHORIZED.into_response())?
                .map_err(|_| StatusCode::UNAUTHORIZED.into_response())?;

            let expected_api_token = config::admin_api_token();

            let token_is_valid = constant_time_eq(
                actual_api_token.0.expose_secret(),
                expected_api_token.0.expose_secret(),
            );

            if !token_is_valid {
                return Err(StatusCode::UNAUTHORIZED.into_response());
            }

            Ok(req)
        }
        .boxed()
    })
}
