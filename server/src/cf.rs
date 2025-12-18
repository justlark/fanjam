use std::fmt::{self, Display};

use secrecy::{ExposeSecret, SecretString};
use serde_json::json;

use crate::{config, env::EnvName, http::RequestBuilder};

#[derive(Debug, Clone)]
pub struct ApiToken(SecretString);

impl From<String> for ApiToken {
    fn from(token: String) -> Self {
        Self(SecretString::from(token))
    }
}

impl ExposeSecret<str> for ApiToken {
    fn expose_secret(&self) -> &str {
        self.0.expose_secret()
    }
}

#[derive(Debug, Clone)]
pub struct CacheTag(String);

impl CacheTag {
    pub fn for_env(env: &EnvName) -> Self {
        Self(format!("env/{}", env))
    }
}

impl Display for CacheTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone)]
pub struct ZoneId(String);

impl From<String> for ZoneId {
    fn from(id: String) -> Self {
        Self(id)
    }
}

impl Display for ZoneId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub struct Client {
    pub api_token: ApiToken,
}

impl Client {
    const API_BASE: &str = "https://api.cloudflare.com/client/v4";

    pub fn new() -> Self {
        Self {
            api_token: config::cloudflare_api_token(),
        }
    }

    fn build_request(&self, method: worker::Method, path: &str) -> anyhow::Result<RequestBuilder> {
        let endpoint = format!("{}{}", Self::API_BASE, path);

        Ok(RequestBuilder::new(method, &endpoint)
            .with_header("Accept", "application/json")
            .with_header(
                "Authorization",
                &format!("Bearer {}", self.api_token.expose_secret()),
            ))
    }

    pub async fn purge_cache(&self, zone_id: &ZoneId, tag: &CacheTag) -> anyhow::Result<()> {
        self.build_request(
            worker::Method::Post,
            &format!("/zones/{zone_id}/purge_cache"),
        )?
        .with_json(&json!({
            "tags": [tag.to_string()]
        }))?
        .exec()
        .await?;

        Ok(())
    }
}
