use std::fmt::{self, Display};

use reqwest::{Method, Url};
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};
use worker::console_log;

use crate::config;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BaseId(String);

impl Display for BaseId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone)]
pub struct ApiToken(SecretString);

impl From<String> for ApiToken {
    fn from(api_token: String) -> Self {
        Self(SecretString::from(api_token))
    }
}

#[derive(Debug)]
pub struct Client {
    client: reqwest::Client,
    api_origin: String,
    api_token: ApiToken,
}

#[derive(Debug, Serialize)]
struct PostBaseRequest {
    title: String,
}

#[derive(Debug, Deserialize)]
struct PostBaseResponse {
    id: String,
}

impl Client {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
            api_origin: config::noco_origin(),
            api_token: config::noco_api_token(),
        }
    }

    fn build_request(&self, method: reqwest::Method, path: &str) -> reqwest::RequestBuilder {
        self.client
            .request(method, format!("{}/api/v2{}", self.api_origin, path))
            .header("Xc-Token", self.api_token.0.expose_secret())
    }

    async fn create_base(&self, title: String) -> anyhow::Result<BaseId> {
        let resp = self
            .build_request(Method::POST, "/meta/bases")
            .json(&PostBaseRequest { title })
            .send()
            .await?;

        let base_id = resp
            .error_for_status()?
            .json::<PostBaseResponse>()
            .await?
            .id;

        console_log!("Created Noco Base with ID: {}", base_id);

        Ok(BaseId(base_id))
    }

    #[worker::send]
    pub async fn create_and_setup_base(&self, title: String) -> anyhow::Result<Url> {
        let base_id = self.create_base(title).await?;
        let app_origin = config::noco_origin();

        Ok(Url::parse(&format!(
            "{app_origin}/dashboard/#/nc/{base_id}"
        ))?)
    }
}
