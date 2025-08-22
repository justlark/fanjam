use std::{borrow::Cow, fmt};

use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};
use worker::Method;

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

// Each tenant instance gets its own Neon project.
//
// https://neon.tech/docs/reference/glossary#project
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ProjectId(String);

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ProjectName(String);

impl From<EnvName> for ProjectName {
    fn from(env_name: EnvName) -> Self {
        Self(env_name.to_string())
    }
}

// Like a git branch, but for the data in Postgres. This is a concept specific to our Postgres
// provider.
//
// https://neon.tech/docs/reference/glossary#branch
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct BranchId(String);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct BranchName(Cow<'static, str>);

impl BranchName {
    pub const fn new(name: &'static str) -> Self {
        Self(Cow::Borrowed(name))
    }
}

impl fmt::Display for BranchName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for BranchName {
    fn from(branch_name: String) -> Self {
        Self(Cow::Owned(branch_name))
    }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackupKind {
    Deletion,
    Deployment,
    Migration,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackupBranch {
    Checkpoint,
    Deployment,
    Migration,
    BaseDeletion,
    MigrationRollback,
    BaseDeletionRollback,
    BaseCreationRollback,
    ManualRestore,
}

impl BackupBranch {
    pub fn name(&self) -> BranchName {
        match self {
            BackupBranch::Checkpoint => BranchName::new("noco-checkpoint"),
            BackupBranch::Deployment => BranchName::new("noco-pre-deployment"),
            BackupBranch::BaseDeletion => BranchName::new("noco-pre-base-deletion"),
            BackupBranch::Migration => BranchName::new("noco-pre-migration"),
            BackupBranch::MigrationRollback => BranchName::new("noco-pre-migration-rollback"),
            BackupBranch::BaseDeletionRollback => {
                BranchName::new("noco-pre-base-deletion-rollback")
            }
            BackupBranch::BaseCreationRollback => {
                BranchName::new("noco-pre-base-creation-rollback")
            }
            BackupBranch::ManualRestore => BranchName::new("noco-pre-manual-restore"),
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
            .with_header("Accept", "application/json")
            .with_header(
                "Authorization",
                &format!("Bearer {}", self.api_token.expose_secret()),
            ))
    }

    async fn lookup_project(&self, project_name: &ProjectName) -> anyhow::Result<ProjectId> {
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
            .with_param("search", &project_name.0)
            .fetch::<GetProjectListResponse>()
            .await?
            .projects
            .into_iter()
            .find(|project| project_name.0 == project.name)
            .ok_or_else(|| anyhow::anyhow!("No Neon project found with name {}", &project_name.0))?
            .id
            .clone();

        Ok(project_id)
    }

    async fn create_branch(
        &self,
        project_id: &ProjectId,
        parent_id: &BranchId,
        branch_name: &BranchName,
        branch_type: BranchType,
    ) -> anyhow::Result<BranchId> {
        #[derive(Debug, Serialize)]
        struct BranchRequestObj {
            name: BranchName,
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
            .with_json(&PostBranchRequest {
                branch: BranchRequestObj {
                    name: branch_name.clone(),
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
        branch_name: &BranchName,
    ) -> anyhow::Result<Option<BranchId>> {
        #[derive(Debug, Deserialize)]
        struct GetBranchListResponse {
            branches: Vec<GetBranchResponse>,
        }

        #[derive(Debug, Deserialize)]
        struct GetBranchResponse {
            id: BranchId,
            name: BranchName,
        }

        let branch_id = self
            .build_request(
                Method::Get,
                &format!("/projects/{}/branches", &project_id.0),
            )?
            .with_param("search", &branch_name.0)
            .fetch::<GetBranchListResponse>()
            .await?
            .branches
            .into_iter()
            .find(|branch| branch_name == &branch.name)
            .map(|branch| branch.id);

        Ok(branch_id)
    }

    async fn delete_branch_with_name(
        &self,
        project_id: &ProjectId,
        branch_name: &BranchName,
    ) -> anyhow::Result<()> {
        let branch_id = self.lookup_branch(project_id, branch_name).await?;

        if let Some(branch_id) = branch_id {
            self.delete_branch(project_id, &branch_id).await?;
        }

        Ok(())
    }

    async fn lookup_default_branch(&self, project_id: &ProjectId) -> anyhow::Result<BranchId> {
        let default_branch_name = config::neon_default_branch_name();
        self.lookup_branch(project_id, &default_branch_name)
            .await?
            .ok_or_else(|| {
                anyhow::anyhow!("No Neon branch found with name {}", &default_branch_name)
            })
    }

    async fn update_branch(
        &self,
        project_id: &ProjectId,
        branch_id: &BranchId,
        new_name: &BranchName,
        is_protected: bool,
    ) -> anyhow::Result<()> {
        #[derive(Debug, Serialize)]
        struct PatchBranchRequest {
            name: BranchName,
            protected: bool,
        }

        self.build_request(
            Method::Patch,
            &format!("/projects/{}/branches/{}", &project_id.0, &branch_id.0,),
        )?
        .with_json(&PatchBranchRequest {
            name: new_name.clone(),
            protected: is_protected,
        })?
        .exec()
        .await?;

        Ok(())
    }

    async fn restore_to_branch(
        &self,
        project_id: &ProjectId,
        to: &BranchId,
        preserve_branch_name: &BranchName,
    ) -> anyhow::Result<()> {
        let default_branch_id = self.lookup_default_branch(project_id).await?;
        let default_branch_name = config::neon_default_branch_name();

        // We kinda just need to hope that all these operations succeed, otherwise the Neon project
        // will be left in a weird state.

        self.build_request(
            Method::Post,
            &format!(
                "/projects/{}/branches/{}/set_as_default",
                &project_id.0, &to.0,
            ),
        )?
        .exec()
        .await?;

        self.delete_branch_with_name(project_id, preserve_branch_name)
            .await?;

        self.update_branch(project_id, &default_branch_id, preserve_branch_name, false)
            .await?;

        self.update_branch(project_id, to, &default_branch_name, true)
            .await?;

        // Otherwise the next time we call this method to restore the default branch, it would fail
        // because you cannot delete a branch with child branches.
        self.delete_child_branches(project_id, &default_branch_id)
            .await?;

        Ok(())
    }

    async fn delete_child_branches(
        &self,
        project_id: &ProjectId,
        parent_id: &BranchId,
    ) -> anyhow::Result<()> {
        #[derive(Debug, Deserialize)]
        struct BranchResponseObj {
            id: BranchId,
            parent_id: BranchId,
        }

        #[derive(Debug, Deserialize)]
        struct GetBranchesResponse {
            branches: Vec<BranchResponseObj>,
        }

        let child_branch_ids = self
            .build_request(
                Method::Get,
                &format!("/projects/{}/branches", &project_id.0),
            )?
            .fetch::<GetBranchesResponse>()
            .await?
            .branches
            .into_iter()
            .filter(|branch| &branch.parent_id == parent_id)
            .map(|branch| branch.id)
            .collect::<Vec<_>>();

        for child_branch_id in child_branch_ids {
            self.delete_branch(project_id, &child_branch_id).await?;
        }

        Ok(())
    }

    #[worker::send]
    pub async fn create_backup(
        &self,
        project_name: &ProjectName,
        branch: BackupBranch,
    ) -> anyhow::Result<BranchId> {
        let project_id = self.lookup_project(project_name).await?;
        let default_branch_id = self.lookup_default_branch(&project_id).await?;

        self.delete_branch_with_name(&project_id, &branch.name())
            .await?;

        let backup_branch_id = self
            .create_branch(
                &project_id,
                &default_branch_id,
                &branch.name(),
                BranchType::ReadOnly,
            )
            .await?;

        Ok(backup_branch_id)
    }

    #[worker::send]
    pub async fn restore_backup(
        &self,
        project_name: &ProjectName,
        backup_kind: BackupKind,
    ) -> anyhow::Result<()> {
        let project_id = self.lookup_project(project_name).await?;

        let source_branch = match backup_kind {
            BackupKind::Deletion => BackupBranch::BaseDeletion,
            BackupKind::Deployment => BackupBranch::Deployment,
            BackupKind::Migration => BackupBranch::Migration,
        };

        let source_branch_id = self
            .lookup_branch(&project_id, &source_branch.name())
            .await?
            .ok_or_else(|| {
                anyhow::anyhow!("no Neon branch found with name {}", &source_branch.name())
            })?;

        self.restore_to_branch(
            &project_id,
            &source_branch_id,
            &BackupBranch::ManualRestore.name(),
        )
        .await?;

        Ok(())
    }

    pub async fn with_rollback<T, Fut, Func>(
        &self,
        project_name: &ProjectName,
        preserve_branch: &BackupBranch,
        f: Func,
    ) -> anyhow::Result<T>
    where
        Fut: Future<Output = anyhow::Result<T>>,
        Func: FnOnce() -> Fut,
    {
        let project_id = self.lookup_project(project_name).await?;
        let backup_branch_id = self
            .create_backup(project_name, BackupBranch::Checkpoint)
            .await?;

        match f().await {
            Ok(result) => Ok(result),
            Err(err) => {
                self.restore_to_branch(&project_id, &backup_branch_id, &preserve_branch.name())
                    .await?;

                Err(anyhow::anyhow!(err))
            }
        }
    }
}
