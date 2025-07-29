use std::time::Duration;

use axum::http::StatusCode;
use secrecy::{ExposeSecret, SecretString};
use worker::{Method, Url};

use crate::http::RequestBuilder;

#[derive(Debug, Clone)]
pub struct ApiToken(SecretString);

impl From<String> for ApiToken {
    fn from(api_token: String) -> Self {
        Self(SecretString::from(api_token))
    }
}

impl ExposeSecret<str> for ApiToken {
    fn expose_secret(&self) -> &str {
        self.0.expose_secret()
    }
}

#[derive(Debug)]
pub struct Client {
    dash_origin: Url,
    api_token: ApiToken,
}

impl Client {
    pub fn new(dash_origin: Url, api_token: ApiToken) -> Self {
        Self {
            dash_origin,
            api_token,
        }
    }

    // Once it's stable, we can migrate to v3 of the NocoDB API.
    pub fn build_request(&self, method: Method, path: &str) -> RequestBuilder {
        let endpoint = format!("{}api/v2{}", self.dash_origin, path);

        // We retry requests that return a 404 Not Found. If the Fly Machine hosting the NocoDB
        // instance is in a stopped state when the request is made, it may return a 404 until it
        // finishes starting up.
        //
        // Fly Machines also have a "suspended" state which starts up much quicker, but in my
        // testing that doesn't seem to play nicely with NocoDB.
        RequestBuilder::new(method, &endpoint)
            .with_header("Xc-Token", self.api_token.0.expose_secret())
            .with_header("Accept", "application/json")
            .with_retry(&[StatusCode::NOT_FOUND], 2, Duration::from_millis(500))
    }
}
