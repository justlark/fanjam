use reqwest::Url;
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};
use worker::console_log;

use crate::config;

const TEMP_ROLLBACK_CHILD_BRANCH_NAME: &str = "temp-rollback";

#[derive(Debug, Clone, Serialize)]
struct Lsn(String);

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
    let status = resp.status();
    let url = resp.url().to_string();

    if status.is_client_error() || status.is_server_error() {
        return Err(anyhow::anyhow!(
            "{} for ({}) with response body:\n{}",
            status,
            url,
            resp.text()
                .await
                .unwrap_or("Could not decode response body".to_string()),
        ));
    }

    Ok(resp)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BranchType {
    ReadOnly,
    #[allow(unused)]
    ReadWrite,
}

impl BranchType {
    fn as_api(&self) -> &str {
        match self {
            BranchType::ReadOnly => "read_only",
            BranchType::ReadWrite => "read_write",
        }
    }
}

#[derive(Debug)]
pub struct Client {
    client: reqwest::Client,
    api_token: ApiToken,
}

impl Client {
    const API_BASE: &str = "https://console.neon.tech/api/v2";

    pub fn new() -> Self {
        let api_token = config::neon_api_token();
        let client = reqwest::Client::new();

        Self { client, api_token }
    }

    fn build_request(
        &self,
        method: reqwest::Method,
        path: &str,
        params: &[(&str, &str)],
    ) -> anyhow::Result<reqwest::RequestBuilder> {
        let endpoint = Url::parse_with_params(&format!("{}{}", Self::API_BASE, path), params)?;

        console_log!("{} {}", method, &endpoint);

        Ok(self
            .client
            .request(method, endpoint)
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .bearer_auth(self.api_token.expose_secret()))
    }

    async fn lookup_project(&self, project_name: &str) -> anyhow::Result<ProjectId> {
        #[derive(Debug, Deserialize)]
        struct GetProjectListResponse {
            projects: Vec<GetProjectResponse>,
        }

        #[derive(Debug, Deserialize)]
        struct GetProjectResponse {
            id: String,
        }

        let org_id = config::neon_org_id();

        let resp = self
            .build_request(
                reqwest::Method::GET,
                "/projects",
                &[
                    ("org_id", &org_id),
                    ("limit", "1"),
                    ("search", project_name),
                ],
            )?
            .send()
            .await?;

        let project_id = check_status(resp)
            .await?
            .json::<GetProjectListResponse>()
            .await?
            .projects
            .first()
            .ok_or_else(|| anyhow::anyhow!("No Neon project found with name {}", &project_name))?
            .id
            .clone();

        Ok(ProjectId(project_id))
    }

    async fn create_branch(
        &self,
        project_id: &ProjectId,
        parent_id: &BranchId,
        branch_name: String,
        branch_type: BranchType,
    ) -> anyhow::Result<BranchId> {
        #[derive(Debug, Serialize)]
        struct BranchRequestObj {
            name: String,
            parent_id: String,
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
                &format!("/projects/{}/branches", project_id.0),
                &[],
            )?
            .json(&PostBranchRequest {
                branch: BranchRequestObj {
                    name: branch_name,
                    parent_id: parent_id.0.clone(),
                },
                endpoints: vec![EndpointRequestObj {
                    r#type: branch_type.as_api().to_string(),
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

    async fn delete_branch(
        &self,
        project_id: &ProjectId,
        branch_id: &BranchId,
    ) -> anyhow::Result<bool> {
        let resp = self
            .build_request(
                reqwest::Method::DELETE,
                &format!("/projects/{}/branches/{}", &project_id.0, &branch_id.0),
                &[],
            )?
            .send()
            .await?;

        check_status(resp).await?;

        Ok(true)
    }

    async fn lookup_branch(
        &self,
        project_id: &ProjectId,
        branch_name: String,
    ) -> anyhow::Result<Option<BranchId>> {
        #[derive(Debug, Deserialize)]
        struct GetBranchListResponse {
            branches: Vec<GetBranchResponse>,
        }

        #[derive(Debug, Deserialize)]
        struct GetBranchResponse {
            id: String,
        }

        let resp = self
            .build_request(
                reqwest::Method::GET,
                &format!("/projects/{}/branches", &project_id.0),
                &[("limit", "1"), ("search", &branch_name)],
            )?
            .send()
            .await?;

        let branch_id = check_status(resp)
            .await?
            .json::<GetBranchListResponse>()
            .await?
            .branches
            .first()
            .map(|branch| BranchId(branch.id.clone()));

        Ok(branch_id)
    }

    async fn delete_branch_with_name(
        &self,
        project_id: &ProjectId,
        branch_name: String,
    ) -> anyhow::Result<()> {
        let branch_id = self.lookup_branch(project_id, branch_name).await?;

        if let Some(branch_id) = branch_id {
            self.delete_branch(project_id, &branch_id).await?;
        }

        Ok(())
    }

    async fn lookup_default_branch(&self, project_id: &ProjectId) -> anyhow::Result<BranchId> {
        let default_branch_name = config::neon_default_branch_name();
        self.lookup_branch(project_id, default_branch_name.to_string())
            .await?
            .ok_or_else(|| {
                anyhow::anyhow!("No Neon branch found with name {}", &default_branch_name,)
            })
    }

    async fn restore_to_lsn(
        &self,
        project_id: &ProjectId,
        branch_id: &BranchId,
        source_lsn: &Lsn,
        preserve_branch_name: String,
    ) -> anyhow::Result<()> {
        #[derive(Debug, Serialize)]
        struct PostRestoreRequest {
            source_branch_id: String,
            source_lsn: String,
            preserve_under_name: String,
        }

        let resp = self
            .build_request(
                reqwest::Method::POST,
                &format!(
                    "/projects/{}/branches/{}/restore",
                    &project_id.0, &branch_id.0,
                ),
                &[],
            )?
            .json(&PostRestoreRequest {
                source_branch_id: branch_id.0.clone(),
                source_lsn: source_lsn.0.clone(),
                preserve_under_name: preserve_branch_name,
            })
            .send()
            .await?;

        check_status(resp).await?;

        Ok(())
    }

    async fn get_lsn(&self, project_id: &ProjectId, branch_id: &BranchId) -> anyhow::Result<Lsn> {
        #[derive(Debug, Deserialize)]
        struct BranchResponseObj {
            parent_lsn: String,
        }

        #[derive(Debug, Deserialize)]
        struct GetBranchResponse {
            branch: BranchResponseObj,
        }

        let resp = self
            .build_request(
                reqwest::Method::GET,
                &format!("/projects/{}/branches/{}", &project_id.0, &branch_id.0,),
                &[],
            )?
            .send()
            .await?;

        let lsn = check_status(resp)
            .await?
            .json::<GetBranchResponse>()
            .await?
            .branch
            .parent_lsn;

        Ok(Lsn(lsn))
    }

    pub async fn with_rollback<T, Fut, Func>(
        &self,
        project_name: &str,
        preserve_name: String,
        f: Func,
    ) -> anyhow::Result<T>
    where
        Fut: Future<Output = Result<T, anyhow::Error>>,
        Func: FnOnce() -> Fut,
    {
        let project_id = self.lookup_project(project_name).await?;
        let default_branch_id = self.lookup_default_branch(&project_id).await?;

        // Just in case deleting the branch failed at any point in the past.
        self.delete_branch_with_name(&project_id, TEMP_ROLLBACK_CHILD_BRANCH_NAME.to_string())
            .await?;

        // There doesn't seem to be an API to get the LSN of the head of the *current* branch--only
        // the LSN of the parent branch. So we create a temporary child branch, get the LSN of its
        // parent, and then delete it.
        let temp_child_branch_id = self
            .create_branch(
                &project_id,
                &default_branch_id,
                TEMP_ROLLBACK_CHILD_BRANCH_NAME.to_string(),
                BranchType::ReadOnly,
            )
            .await?;

        let source_lsn = self.get_lsn(&project_id, &temp_child_branch_id).await?;

        self.delete_branch(&project_id, &temp_child_branch_id)
            .await?;

        match f().await {
            Ok(result) => Ok(result),
            Err(err) => {
                self.delete_branch_with_name(&project_id, preserve_name.clone())
                    .await?;

                self.restore_to_lsn(&project_id, &default_branch_id, &source_lsn, preserve_name)
                    .await?;

                Err(anyhow::anyhow!(err))
            }
        }
    }

    #[worker::send]
    pub async fn create_backup(
        &self,
        project_name: &str,
        branch_name: String,
    ) -> anyhow::Result<BranchId> {
        let project_id = self.lookup_project(project_name).await?;
        let default_branch_id = self.lookup_default_branch(&project_id).await?;

        self.delete_branch_with_name(&project_id, branch_name.clone())
            .await?;

        let backup_branch_id = self
            .create_branch(
                &project_id,
                &default_branch_id,
                branch_name,
                BranchType::ReadOnly,
            )
            .await?;

        Ok(backup_branch_id)
    }
}
