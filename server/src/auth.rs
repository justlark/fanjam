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

impl ExposeSecret<[u8]> for ApiToken {
    fn expose_secret(&self) -> &[u8] {
        self.0.expose_secret()
    }
}

type BoxFutureResponseResult<'a> = BoxFuture<'a, Result<Request<Body>, Response<Body>>>;

pub fn admin_auth_layer<'a>()
-> AsyncRequireAuthorizationLayer<impl Fn(Request<Body>) -> BoxFutureResponseResult<'a> + Clone> {
    bearer_auth_layer(|| Some(config::admin_api_token()))
}

// Configuring this worker with a token for webhooks is optional, because it
// is currently only used for push notifications. If a token is not
// configured, we reject with 503 Service Unavailable so that NocoDB doesn't
// keep trying.
pub fn noco_webhook_auth_layer<'a>()
-> AsyncRequireAuthorizationLayer<impl Fn(Request<Body>) -> BoxFutureResponseResult<'a> + Clone> {
    bearer_auth_layer(config::noco_webhook_token)
}

fn bearer_auth_layer<'a, F>(
    expected: F,
) -> AsyncRequireAuthorizationLayer<impl Fn(Request<Body>) -> BoxFutureResponseResult<'a> + Clone>
where
    F: Fn() -> Option<ApiToken> + Clone + Send + Sync + 'static,
{
    AsyncRequireAuthorizationLayer::new(move |req: Request<Body>| {
        let expected = expected.clone();
        async move {
            let expected_api_token =
                expected().ok_or_else(|| StatusCode::SERVICE_UNAVAILABLE.into_response())?;

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
