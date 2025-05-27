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

        RequestBuilder::new(method, &endpoint)
            .with_header("Xc-Token", self.api_token.0.expose_secret())
            .with_header("Accept", "application/json")
    }
}
