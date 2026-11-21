use anyhow::Context;
use axum::{
    body::Body,
    http::{HeaderMap, Request, Response, StatusCode, header::AUTHORIZATION},
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

// Parse a base64-encoded bearer token from the `Authorization` header, if there is a valid one.
pub fn bearer_token(headers: &HeaderMap) -> Option<ApiToken> {
    headers
        .get(AUTHORIZATION)?
        .to_str()
        .ok()?
        .strip_prefix(BEARER_PREFIX)
        .and_then(|token| ApiToken::try_from(token).ok())
}

// Check a bearer token against the app passwords configured for an environment. When there are no
// passwords, the app is open to everyone, so any request (with or without a token) is allowed.
pub fn token_matches_any<'a>(
    token: Option<&ApiToken>,
    passwords: impl IntoIterator<Item = &'a str>,
) -> bool {
    let mut passwords = passwords.into_iter().peekable();

    if passwords.peek().is_none() {
        return true;
    }

    let Some(token) = token else {
        return false;
    };

    // Deliberately check every password rather than stopping at the first match.
    passwords.fold(false, |matched, password| {
        constant_time_eq(token.expose_secret(), password.as_bytes()) | matched
    })
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

            let actual_api_token = bearer_token(req.headers())
                .ok_or_else(|| StatusCode::UNAUTHORIZED.into_response())?;

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

#[cfg(test)]
mod tests {
    use super::*;

    fn token(password: &str) -> ApiToken {
        ApiToken::from(password.as_bytes().to_vec())
    }

    #[test]
    fn no_passwords_allows_everyone() {
        assert!(token_matches_any(None, []));
        assert!(token_matches_any(Some(&token("anything")), []));
    }

    #[test]
    fn missing_token_is_rejected() {
        assert!(!token_matches_any(None, ["hunter2"]));
    }

    #[test]
    fn any_password_is_accepted() {
        let passwords = ["hunter2", "correct horse battery staple"];
        assert!(token_matches_any(Some(&token("hunter2")), passwords));
        assert!(token_matches_any(
            Some(&token("correct horse battery staple")),
            passwords
        ));
    }

    #[test]
    fn wrong_password_is_rejected() {
        assert!(!token_matches_any(Some(&token("hunter3")), ["hunter2"]));
        assert!(!token_matches_any(Some(&token("")), ["hunter2"]));
    }

    #[test]
    fn parses_base64_bearer_token() {
        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, "Bearer aHVudGVyMg==".parse().unwrap());
        let parsed = bearer_token(&headers).expect("expected a token");
        assert_eq!(parsed.expose_secret(), b"hunter2");

        headers.insert(AUTHORIZATION, "Basic aHVudGVyMg==".parse().unwrap());
        assert!(bearer_token(&headers).is_none());

        headers.insert(AUTHORIZATION, "Bearer not base64!".parse().unwrap());
        assert!(bearer_token(&headers).is_none());
    }
}
