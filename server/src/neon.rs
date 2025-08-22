use std::{borrow::Cow, fmt, iter};

use axum::http::StatusCode;
use rand::Rng;
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

// A point-in-time snapshot of a branch.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SnapshotId(String);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct BranchName(Cow<'static, str>);

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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SnapshotName(Cow<'static, str>);

impl SnapshotName {
    pub const fn new(name: &'static str) -> Self {
        Self(Cow::Borrowed(name))
    }
}

impl fmt::Display for SnapshotName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for SnapshotName {
    fn from(snapshot_name: String) -> Self {
        Self(Cow::Owned(snapshot_name))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackupKind {
    Deletion,
    Deployment,
    Migration,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackupSnapshot {
    Checkpoint,
    Deployment,
    Migration,
    BaseDeletion,
    ManualRestore,
    Archived,
}

impl BackupSnapshot {
    const ARCHIVED_PREFIX: &'static str = "noco-archived-";

    pub fn name(&self) -> SnapshotName {
        match self {
            BackupSnapshot::Checkpoint => SnapshotName::new("noco-checkpoint"),
            BackupSnapshot::Deployment => SnapshotName::new("noco-pre-deployment"),
            BackupSnapshot::BaseDeletion => SnapshotName::new("noco-pre-base-deletion"),
            BackupSnapshot::Migration => SnapshotName::new("noco-pre-migration"),
            BackupSnapshot::ManualRestore => SnapshotName::new("noco-pre-manual-restore"),
            BackupSnapshot::Archived => {
                const LEN: usize = 4;
                const POOL: &str = "0123456789";

                let mut rng = rand::rng();

                let prefix = Self::ARCHIVED_PREFIX;
                let suffix = iter::repeat_with(|| {
                    let idx = rng.random_range(0..POOL.len());
                    POOL.chars().nth(idx).unwrap()
                })
                .take(LEN)
                .collect::<String>();

                SnapshotName(Cow::Owned(format!("{prefix}{suffix}")))
            }
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

    pub async fn lookup_project(&self, project_name: &ProjectName) -> anyhow::Result<ProjectId> {
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

    async fn try_delete_snapshot(
        &self,
        project_id: &ProjectId,
        snapshot_id: &SnapshotId,
    ) -> anyhow::Result<()> {
        self.build_request(
            Method::Delete,
            &format!("/projects/{}/snapshots/{}", &project_id.0, &snapshot_id.0),
        )?
        .allow_status(StatusCode::BAD_REQUEST)
        .exec()
        .await?;

        Ok(())
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

    async fn delete_snapshot_restore_branches(&self, project_id: &ProjectId) -> anyhow::Result<()> {
        #[derive(Debug, Deserialize)]
        struct GetBranchListResponse {
            branches: Vec<GetBranchResponse>,
        }

        #[derive(Debug, Deserialize)]
        struct GetBranchResponse {
            id: BranchId,
            restored_from: Option<SnapshotId>,
            default: bool,
        }

        let branch_ids = self
            .build_request(
                Method::Get,
                &format!("/projects/{}/branches", &project_id.0),
            )?
            .fetch::<GetBranchListResponse>()
            .await?
            .branches
            .into_iter()
            .filter(|branch| branch.restored_from.is_some() && !branch.default)
            .map(|branch| branch.id);

        for branch_id in branch_ids {
            self.delete_branch(project_id, &branch_id).await?;
        }

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

    async fn lookup_snapshot(
        &self,
        project_id: &ProjectId,
        snapshot_name: &SnapshotName,
    ) -> anyhow::Result<Option<SnapshotId>> {
        #[derive(Debug, Deserialize)]
        struct GetSnapshotListResponse {
            snapshots: Vec<GetSnapshotResponse>,
        }

        #[derive(Debug, Deserialize)]
        struct GetSnapshotResponse {
            id: SnapshotId,
            name: SnapshotName,
        }

        let snapshot_id = self
            .build_request(
                Method::Get,
                &format!("/projects/{}/snapshots", &project_id.0),
            )?
            .fetch::<GetSnapshotListResponse>()
            .await?
            .snapshots
            .into_iter()
            .find(|snapshot| snapshot_name == &snapshot.name)
            .map(|snapshot| snapshot.id);

        Ok(snapshot_id)
    }

    async fn list_archived_snapshots(
        &self,
        project_id: &ProjectId,
    ) -> anyhow::Result<Vec<SnapshotId>> {
        #[derive(Debug, Deserialize)]
        struct GetSnapshotListResponse {
            snapshots: Vec<GetSnapshotResponse>,
        }

        #[derive(Debug, Deserialize)]
        struct GetSnapshotResponse {
            id: SnapshotId,
            name: SnapshotName,
        }

        let snapshot_ids = self
            .build_request(
                Method::Get,
                &format!("/projects/{}/snapshots", &project_id.0),
            )?
            .fetch::<GetSnapshotListResponse>()
            .await?
            .snapshots
            .into_iter()
            .filter(|snapshot| snapshot.name.0.starts_with(BackupSnapshot::ARCHIVED_PREFIX))
            .map(|snapshot| snapshot.id)
            .collect();

        Ok(snapshot_ids)
    }

    async fn garbage_collect_archived_snapshots(
        &self,
        project_id: &ProjectId,
    ) -> anyhow::Result<()> {
        let archived_snapshots = self.list_archived_snapshots(project_id).await?;

        for snapshot_id in archived_snapshots {
            self.try_delete_snapshot(project_id, &snapshot_id).await?;
        }

        Ok(())
    }

    async fn soft_delete_snapshot_with_name(
        &self,
        project_id: &ProjectId,
        snapshot_name: &SnapshotName,
    ) -> anyhow::Result<()> {
        let snapshot_id = self.lookup_snapshot(project_id, snapshot_name).await?;

        if let Some(snapshot_id) = snapshot_id {
            self.rename_snapshot(project_id, &snapshot_id, &BackupSnapshot::Archived.name())
                .await?;
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

    async fn rename_snapshot(
        &self,
        project_id: &ProjectId,
        snapshot_id: &SnapshotId,
        new_name: &SnapshotName,
    ) -> anyhow::Result<()> {
        #[derive(Debug, Serialize)]
        struct PatchSnapshotRequest {
            snapshot: SnapshotRequestObj,
        }

        #[derive(Debug, Serialize)]
        struct SnapshotRequestObj {
            name: SnapshotName,
        }

        self.build_request(
            Method::Patch,
            &format!("/projects/{}/snapshots/{}", &project_id.0, &snapshot_id.0),
        )?
        .with_json(&PatchSnapshotRequest {
            snapshot: SnapshotRequestObj {
                name: new_name.clone(),
            },
        })?
        .exec()
        .await?;

        Ok(())
    }

    async fn restore_to_snapshot(
        &self,
        project_id: &ProjectId,
        branch: &BranchId,
        to: &SnapshotId,
    ) -> anyhow::Result<()> {
        #[derive(Debug, Serialize)]
        struct PostRestoreSnapshotRequest {
            target_branch_id: BranchId,
            finalize_restore: bool,
        }

        self.delete_snapshot_restore_branches(project_id).await?;

        self.build_request(
            Method::Post,
            &format!("/projects/{}/snapshots/{}/restore", &project_id.0, &to.0,),
        )?
        .with_json(&PostRestoreSnapshotRequest {
            target_branch_id: branch.clone(),
            finalize_restore: true,
        })?
        .exec()
        .await?;

        Ok(())
    }

    pub async fn create_backup(
        &self,
        project_id: &ProjectId,
        snapshot: BackupSnapshot,
    ) -> anyhow::Result<SnapshotId> {
        let default_branch_id = self.lookup_default_branch(project_id).await?;

        self.soft_delete_snapshot_with_name(project_id, &snapshot.name())
            .await?;

        self.garbage_collect_archived_snapshots(project_id).await?;

        let backup_snapshot_id = self
            .create_snapshot(project_id, &default_branch_id, &snapshot.name())
            .await?;

        Ok(backup_snapshot_id)
    }

    async fn create_snapshot(
        &self,
        project_id: &ProjectId,
        branch: &BranchId,
        name: &SnapshotName,
    ) -> anyhow::Result<SnapshotId> {
        #[derive(Debug, Deserialize)]
        struct SnapshotResponseObj {
            id: SnapshotId,
        }

        #[derive(Debug, Deserialize)]
        struct PostSnapshotResponse {
            snapshot: SnapshotResponseObj,
        }

        let snapshot_id = self
            .build_request(
                Method::Post,
                &format!(
                    "/projects/{}/branches/{}/snapshot",
                    &project_id.0, &branch.0
                ),
            )?
            .with_param("name", &name.to_string())
            .fetch::<PostSnapshotResponse>()
            .await?
            .snapshot
            .id;

        Ok(snapshot_id)
    }

    #[worker::send]
    pub async fn restore_backup(
        &self,
        project_name: &ProjectName,
        backup_kind: BackupKind,
    ) -> anyhow::Result<()> {
        let project_id = self.lookup_project(project_name).await?;
        let default_branch_id = self.lookup_default_branch(&project_id).await?;

        let source_snapshot = match backup_kind {
            BackupKind::Deletion => BackupSnapshot::BaseDeletion,
            BackupKind::Deployment => BackupSnapshot::Deployment,
            BackupKind::Migration => BackupSnapshot::Migration,
        };

        self.create_backup(&project_id, BackupSnapshot::ManualRestore)
            .await?;

        let source_snapshot_id = self
            .lookup_snapshot(&project_id, &source_snapshot.name())
            .await?
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "no Neon snapshot found with name {}",
                    &source_snapshot.name()
                )
            })?;

        self.restore_to_snapshot(&project_id, &default_branch_id, &source_snapshot_id)
            .await?;

        Ok(())
    }

    pub async fn with_rollback<T, Fut, Func>(
        &self,
        project_name: &ProjectName,
        f: Func,
    ) -> anyhow::Result<T>
    where
        Fut: Future<Output = anyhow::Result<T>>,
        Func: FnOnce() -> Fut,
    {
        let project_id = self.lookup_project(project_name).await?;
        let default_branch_id = self.lookup_default_branch(&project_id).await?;

        let backup_snapshot_id = self
            .create_backup(&project_id, BackupSnapshot::Checkpoint)
            .await?;

        match f().await {
            Ok(result) => Ok(result),
            Err(err) => {
                self.restore_to_snapshot(&project_id, &default_branch_id, &backup_snapshot_id)
                    .await?;

                Err(anyhow::anyhow!(err))
            }
        }
    }
}
