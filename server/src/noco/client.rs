use reqwest::Url;
use secrecy::{ExposeSecret, SecretString};
use worker::console_log;

use crate::http;

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
    client: reqwest::Client,
    dash_origin: Url,
    api_token: ApiToken,
}

impl Client {
    pub fn new(dash_origin: Url, api_token: ApiToken) -> Self {
        Self {
            client: http::get_client(),
            dash_origin,
            api_token,
        }
    }

    // Once it's stable, we can migrate to v3 of the NocoDB API.
    pub fn build_request_v2(&self, method: reqwest::Method, path: &str) -> reqwest::RequestBuilder {
        console_log!("{} {}", method, path);

        self.client
            .request(method, format!("{}api/v2{}", self.dash_origin, path))
            .header("Xc-Token", self.api_token.0.expose_secret())
    }
}
