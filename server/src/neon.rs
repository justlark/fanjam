use reqwest::Url;
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};

use crate::{config, env::EnvName};

const API_BASE: &str = "https://console.neon.tech/api/v2";

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

#[derive(Debug, Clone)]
pub struct ProjectId(String);

#[derive(Debug, Clone)]
pub struct BranchId(String);

async fn check_status(resp: reqwest::Response) -> anyhow::Result<reqwest::Response> {
    #[derive(Debug, Deserialize)]
    struct ErrorResponse {
        message: Option<String>,
    }

    let status = resp.status();
    let url = resp.url().to_string();

    if status.is_client_error() || status.is_server_error() {
        let resp = resp.json::<ErrorResponse>().await?;

        return Err(anyhow::anyhow!(
            "Error: {} for ({}) with message ({})",
            status,
            url,
            resp.message.unwrap_or_default(),
        ));
    }

    Ok(resp)
}

#[derive(Debug)]
pub struct Client {
    client: reqwest::Client,
    api_token: ApiToken,
}

impl Client {
    pub fn new() -> Self {
        let api_token = config::neon_api_token();
        let client = reqwest::Client::new();

        Self { client, api_token }
    }

    fn build_request(&self, method: reqwest::Method, path: &str) -> reqwest::RequestBuilder {
        self.client
            .request(method, format!("{}{}", API_BASE, path))
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .bearer_auth(self.api_token.expose_secret())
    }

    pub async fn lookup_project(&self, env_name: &EnvName) -> anyhow::Result<ProjectId> {
        #[derive(Debug, Deserialize)]
        struct GetProjectResponse {
            id: String,
        }

        let org_id = config::neon_org_id();

        let endpoint = Url::parse_with_params(
            "/projects",
            &[
                ("org_id", org_id),
                ("limit", "1".to_string()),
                ("search", env_name.to_string()),
            ],
        )?;

        let resp = self
            .build_request(reqwest::Method::GET, endpoint.as_str())
            .send()
            .await?;

        let project_id = check_status(resp)
            .await?
            .json::<Vec<GetProjectResponse>>()
            .await?
            .first()
            .ok_or_else(|| anyhow::anyhow!("No Neon project found with name {}", &env_name))?
            .id
            .clone();

        Ok(ProjectId(project_id))
    }

    pub async fn create_branch(
        &self,
        project: &ProjectId,
        branch_name: String,
    ) -> anyhow::Result<BranchId> {
        #[derive(Debug, Serialize)]
        struct BranchRequestObj {
            name: String,
        }

        #[derive(Debug, Serialize)]
        struct EndpointRequestObj {
            r#type: String,
        }

        #[derive(Debug, Serialize)]
        struct PostBranchRequest {
            branch: BranchRequestObj,
            endpoints: Vec<EndpointRequestObj>,
        }

        #[derive(Debug, Deserialize)]
        struct BranchResponseObj {
            id: String,
        }

        #[derive(Debug, Deserialize)]
        struct PostBranchResponse {
            branch: BranchResponseObj,
        }

        let resp = self
            .build_request(
                reqwest::Method::POST,
                &format!("/projects/{}/branches", project.0),
            )
            .json(&PostBranchRequest {
                branch: BranchRequestObj { name: branch_name },
                endpoints: vec![EndpointRequestObj {
                    r#type: "read_only".to_string(),
                }],
            })
            .send()
            .await?;

        let branch_id = check_status(resp)
            .await?
            .json::<PostBranchResponse>()
            .await?
            .branch
            .id;

        Ok(BranchId(branch_id))
    }
}
