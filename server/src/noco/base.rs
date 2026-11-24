use axum::http::StatusCode;
use serde::Deserialize;
use serde_json::json;
use worker::Method;

use super::{Client, migrations::BaseId};
use crate::log;

async fn create_empty_base(
    client: &Client,
    title: String,
    env: log::EnvRef<'_>,
) -> anyhow::Result<BaseId> {
    #[derive(Debug, Deserialize)]
    struct PostBaseResponse {
        id: BaseId,
    }

    let base_id = client
        .build_request_v2(Method::Post, "/meta/bases")
        .with_json(&json!({
            "title": title
        }))?
        .fetch::<PostBaseResponse>()
        .await?
        .id;

    log::log(log::Message {
        id: "noco-base-created",
        env,
        msg: &format!("Created Noco base `{}` with ID `{}`", title, base_id),
    });

    Ok(base_id)
}

async fn add_user(
    client: &Client,
    base_id: &BaseId,
    initial_user_email: String,
    env: log::EnvRef<'_>,
) -> anyhow::Result<()> {
    client
        .build_request_v2(Method::Post, &format!("/meta/bases/{base_id}/users"))
        .with_json(&json!({
            "email": initial_user_email,
            "roles": "editor",
        }))?
        .exec()
        .await?;

    log::log(log::Message {
        id: "noco-base-user-added",
        env,
        msg: &format!(
            "Added user `{}` to Noco base `{}`",
            initial_user_email, base_id
        ),
    });

    Ok(())
}

pub async fn create_base(
    client: &Client,
    title: String,
    initial_user_email: String,
    env: log::EnvRef<'_>,
) -> anyhow::Result<BaseId> {
    let base_id = create_empty_base(client, title, env.clone()).await?;
    add_user(client, &base_id, initial_user_email, env).await?;

    Ok(base_id)
}

#[worker::send]
pub async fn delete_base(
    client: &Client,
    base_id: &BaseId,
    env: log::EnvRef<'_>,
) -> anyhow::Result<()> {
    client
        .build_request_v2(Method::Delete, &format!("/meta/bases/{base_id}"))
        .exec()
        .await?;

    log::log(log::Message {
        id: "noco-base-deleted",
        env,
        msg: &format!("Deleted Noco base with ID `{}`", base_id),
    });

    Ok(())
}

#[worker::send]
pub async fn check_base_exists(client: &Client, base_id: &BaseId) -> anyhow::Result<bool> {
    let status_code = client
        .build_request_v2(Method::Get, &format!("/meta/bases/{base_id}"))
        .allow_status(StatusCode::NOT_FOUND)
        .allow_status(StatusCode::UNPROCESSABLE_ENTITY)
        .exec()
        .await?;

    match status_code {
        StatusCode::NOT_FOUND | StatusCode::UNPROCESSABLE_ENTITY => Ok(false),
        _ => Ok(true),
    }
}
