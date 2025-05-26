use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};
use worker::Method;

use crate::{config, http::RequestBuilder};

const TEMP_ROLLBACK_CHILD_BRANCH_NAME: &str = "temp-rollback";

// An LSN (Log Sequence Number) is sort of an index into the history of a branch. It's a point we
// can roll back to. This is a concept specific to our Postgres provider.
//
// https://neon.tech/docs/reference/glossary#lsn
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
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

// Each tenant instance gets its own Neon project.
//
// https://neon.tech/docs/reference/glossary#project
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ProjectId(String);

// Like a git branch, but for the data in Postgres. This is a concept specific to our Postgres
// provider.
//
// https://neon.tech/docs/reference/glossary#branch
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct BranchId(String);

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
    api_token: ApiToken,
}

impl Client {
    const API_BASE: &str = "https://console.neon.tech/api/v2";

    pub fn new() -> Self {
        let api_token = config::neon_api_token();

        Self { api_token }
    }

    fn build_request(&self, method: worker::Method, path: &str) -> anyhow::Result<RequestBuilder> {
        let endpoint = format!("{}{}", Self::API_BASE, path);

        Ok(RequestBuilder::new(method, &endpoint)
            .with_header("Content-Type", "application/json")
            .with_header("Accept", "application/json")
            .with_header(
                "Authorization",
                &format!("Bearer {}", self.api_token.expose_secret()),
            ))
    }

    async fn lookup_project(&self, project_name: &str) -> anyhow::Result<ProjectId> {
        #[derive(Debug, Deserialize)]
        struct GetProjectListResponse {
            projects: Vec<GetProjectResponse>,
        }

        #[derive(Debug, Deserialize)]
        struct GetProjectResponse {
            id: ProjectId,
            name: String,
        }

        let org_id = config::neon_org_id();

        let project_id = self
            .build_request(Method::Get, "/projects")?
            .with_param("org_id", &org_id)
            .with_param("search", project_name)
            .fetch::<GetProjectListResponse>()
            .await?
            .projects
            .into_iter()
            .find(|project| project_name == project.name)
            .ok_or_else(|| anyhow::anyhow!("No Neon project found with name {}", &project_name))?
            .id
            .clone();

        Ok(project_id)
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
            parent_id: BranchId,
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
            id: BranchId,
        }

        #[derive(Debug, Deserialize)]
        struct PostBranchResponse {
            branch: BranchResponseObj,
        }

        let branch_id = self
            .build_request(
                Method::Post,
                &format!("/projects/{}/branches", project_id.0),
            )?
            .with_body(&PostBranchRequest {
                branch: BranchRequestObj {
                    name: branch_name,
                    parent_id: parent_id.clone(),
                },
                endpoints: vec![EndpointRequestObj {
                    r#type: branch_type.as_api().to_string(),
                }],
            })?
            .fetch::<PostBranchResponse>()
            .await?
            .branch
            .id;

        Ok(branch_id)
    }

    async fn delete_branch(
        &self,
        project_id: &ProjectId,
        branch_id: &BranchId,
    ) -> anyhow::Result<()> {
        self.build_request(
            Method::Delete,
            &format!("/projects/{}/branches/{}", &project_id.0, &branch_id.0),
        )?
        .exec()
        .await?;

        Ok(())
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
            id: BranchId,
            name: String,
        }

        let branch_id = self
            .build_request(
                Method::Get,
                &format!("/projects/{}/branches", &project_id.0),
            )?
            .with_param("search", &branch_name)
            .fetch::<GetBranchListResponse>()
            .await?
            .branches
            .into_iter()
            .find(|branch| branch_name == branch.name)
            .map(|branch| branch.id);

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

        self.build_request(
            Method::Post,
            &format!(
                "/projects/{}/branches/{}/restore",
                &project_id.0, &branch_id.0,
            ),
        )?
        .with_body(&PostRestoreRequest {
            source_branch_id: branch_id.0.clone(),
            source_lsn: source_lsn.0.clone(),
            preserve_under_name: preserve_branch_name,
        })?
        .exec()
        .await?;

        Ok(())
    }

    async fn get_lsn(&self, project_id: &ProjectId, branch_id: &BranchId) -> anyhow::Result<Lsn> {
        #[derive(Debug, Deserialize)]
        struct BranchResponseObj {
            parent_lsn: Lsn,
        }

        #[derive(Debug, Deserialize)]
        struct GetBranchResponse {
            branch: BranchResponseObj,
        }

        let lsn = self
            .build_request(
                Method::Get,
                &format!("/projects/{}/branches/{}", &project_id.0, &branch_id.0,),
            )?
            .fetch::<GetBranchResponse>()
            .await?
            .branch
            .parent_lsn;

        Ok(lsn)
    }

    pub async fn with_rollback<T, Fut, Func>(
        &self,
        project_name: &str,
        preserve_name: String,
        f: Func,
    ) -> anyhow::Result<T>
    where
        Fut: Future<Output = anyhow::Result<T>>,
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

        // The alternative to rolling back to an LSN would be to roll back to a timestamp.
        // Timestamps have limited precision and require everyone agree on the exact time, which
        // leaves open the possibility of accidentally rolling back too far or not far enough, even
        // though in practice it would probably be Good Enough.
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
