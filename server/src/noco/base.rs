use reqwest::Method;
use serde::Deserialize;
use serde_json::json;
use worker::console_log;

use crate::noco::client::check_status;

use super::{Client, migrations::BaseId};

async fn create_empty_base(client: &Client, title: String) -> anyhow::Result<BaseId> {
    let resp = client
        .build_request_v2(Method::POST, "/meta/bases")
        .json(&json!({
            "title": title
        }))
        .send()
        .await?;

    #[derive(Debug, Deserialize)]
    struct PostBaseResponse {
        id: BaseId,
    }

    let base_id = check_status(resp)
        .await?
        .json::<PostBaseResponse>()
        .await?
        .id;

    console_log!("Created Noco base `{}` with ID `{}`", title, base_id);

    Ok(base_id)
}

async fn add_user(
    client: &Client,
    base_id: &BaseId,
    initial_user_email: String,
) -> anyhow::Result<()> {
    let resp = client
        .build_request_v3(Method::POST, &format!("/meta/bases/{}/users", base_id))
        .json(&json!({
            "email": initial_user_email,
            "base_role": "editor",
        }))
        .send()
        .await?;

    check_status(resp).await?;

    console_log!(
        "Added user `{}` to Noco base `{}`",
        initial_user_email,
        base_id
    );

    Ok(())
}

pub async fn create_base(
    client: &Client,
    title: String,
    initial_user_email: String,
) -> anyhow::Result<BaseId> {
    let base_id = create_empty_base(client, title).await?;
    add_user(client, &base_id, initial_user_email).await?;

    Ok(base_id)
}
