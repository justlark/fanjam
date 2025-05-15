use reqwest::Url;
use secrecy::{ExposeSecret, SecretString};
use serde::Deserialize;
use worker::console_log;

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

pub async fn check_status(resp: reqwest::Response) -> anyhow::Result<reqwest::Response> {
    #[derive(Debug, Deserialize)]
    struct ErrorResponse {
        msg: Option<String>,
        errors: Option<serde_json::Value>,
    }

    let status = resp.status();
    let url = resp.url().to_string();

    if status.is_client_error() || status.is_server_error() {
        let resp = resp.json::<ErrorResponse>().await?;

        return Err(anyhow::anyhow!(
            "Error: {} for ({}) with message ({})\n{}",
            status,
            url,
            resp.msg.unwrap_or_default(),
            resp.errors.unwrap_or_default(),
        ));
    }

    Ok(resp)
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
            client: reqwest::Client::new(),
            dash_origin,
            api_token,
        }
    }

    // We're building this on top of the new v3 API, but we still need to fall back to the v2 API
    // for some operations that are not yet supported in v3.

    pub fn build_request_v2(&self, method: reqwest::Method, path: &str) -> reqwest::RequestBuilder {
        console_log!("{} {}", method, path);

        self.client
            .request(method, format!("{}api/v2{}", self.dash_origin, path))
            .header("Xc-Token", self.api_token.0.expose_secret())
    }

    pub fn build_request_v3(&self, method: reqwest::Method, path: &str) -> reqwest::RequestBuilder {
        console_log!("{} {}", method, path);

        self.client
            .request(method, format!("{}api/v3{}", self.dash_origin, path))
            .header("Xc-Token", self.api_token.0.expose_secret())
    }
}
