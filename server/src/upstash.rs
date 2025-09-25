//
// For reasons explained in a comment in the Terraform, we currently have Redis caching disabled.
// We're keeping this code checked in in case we need it in the future, but it's disabled.
//

/*
use secrecy::{ExposeSecret, SecretString};
use serde::Deserialize;

use crate::{config, env::EnvName, http::RequestBuilder};

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
    api_token: ApiToken,
    endpoint: String,
}

impl Client {
    pub fn new() -> Self {
        Self {
            endpoint: config::upstash_endpoint().into(),
            api_token: config::upstash_api_token(),
        }
    }

    async fn run<T>(&self, command: &[impl AsRef<str>]) -> anyhow::Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        #[derive(Debug, Deserialize)]
        struct Response<T> {
            result: T,
        }

        let command = command
            .iter()
            .map(|s| s.as_ref().to_string())
            .collect::<Vec<_>>();

        RequestBuilder::new(worker::Method::Post, &self.endpoint)
            .with_header("Accept", "application/json")
            .with_header(
                "Authorization",
                &format!("Bearer {}", self.api_token.expose_secret()),
            )
            .with_json(&command)?
            .fetch::<Response<T>>()
            .await
            .map(|response| response.result)
    }

    async fn unlink_keys(&self, pattern: &str) -> anyhow::Result<()> {
        let mut next_cursor = String::from("0");

        type ScanResponse = (String, Vec<String>);
        type DelResponse = i32;

        loop {
            let (cursor, keys) = self
                .run::<ScanResponse>(&["SCAN", &next_cursor, "MATCH", pattern])
                .await?;

            let del_command = [String::from("UNLINK")]
                .into_iter()
                .chain(keys.into_iter())
                .collect::<Vec<_>>();

            self.run::<DelResponse>(&del_command).await?;

            if cursor == "0" {
                break;
            }

            next_cursor = cursor;
        }

        Ok(())
    }

    #[worker::send]
    pub async fn unlink_noco_keys(&self, env_name: &EnvName) -> anyhow::Result<()> {
        self.unlink_keys(&format!("sparklefish:env:{env_name}:noco:*"))
            .await
    }
}
*/
