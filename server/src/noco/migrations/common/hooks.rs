use base64::prelude::*;
use secrecy::ExposeSecret;
use serde::Deserialize;
use serde_json::json;
use worker::Method;

use crate::{auth::ApiToken, log, noco::Client};

use super::{HookId, TableId};

// The title we put on the hook that sends push notifications for new announcements. Migrations use
// it to find the hook again.
pub const PUSH_NOTIFICATIONS_HOOK_TITLE: &str = "FanJam push notifications";

#[derive(Debug, Deserialize)]
pub struct HookSummary {
    pub id: HookId,
    pub title: Option<String>,
}

pub async fn list_hooks(client: &Client, table_id: &TableId) -> anyhow::Result<Vec<HookSummary>> {
    #[derive(Debug, Deserialize)]
    struct ListHooksResponse {
        list: Vec<HookSummary>,
    }

    Ok(client
        .build_request_v2(Method::Get, &format!("/meta/tables/{table_id}/hooks"))
        .fetch::<ListHooksResponse>()
        .await?
        .list)
}

pub async fn delete_hook(
    client: &Client,
    hook_id: &HookId,
    env: log::EnvRef<'_>,
) -> anyhow::Result<()> {
    client
        .build_request_v2(Method::Delete, &format!("/meta/hooks/{hook_id}"))
        .exec()
        .await?;

    log::log(log::Message {
        id: "noco-hook-deleted",
        env,
        msg: &format!("Deleted Noco hook with ID `{}`", hook_id),
    });

    Ok(())
}

// Create a hook that POSTs each newly inserted row in `table_id` to `url` as JSON, authenticated
// with `token` as a bearer token.
pub async fn create_url_hook(
    client: &Client,
    table_id: &TableId,
    title: &str,
    url: &str,
    token: &ApiToken,
    env: log::EnvRef<'_>,
) -> anyhow::Result<()> {
    let bearer = BASE64_STANDARD.encode(token.expose_secret());

    let notification = serde_json::to_string(&json!({
        "type": "URL",
        "include_user": false,
        "payload": {
            "method": "POST",
            "path": url,
            "auth": "",
            "body": "{{ json event }}",
            "headers": [
                {
                    "enabled": true,
                    "name": "Authorization",
                    "value": format!("Bearer {bearer}"),
                },
                {
                    "enabled": true,
                    "name": "Content-Type",
                    "value": "application/json",
                },
            ],
            "parameters": [],
        },
        "trigger_form": false,
    }))?;

    let body = json!({
        "title": title,
        "event": "after",
        "operation": ["insert"],
        "version": "v3",
        "active": true,
        "notification": notification,
    });

    client
        .build_request_v2(Method::Post, &format!("/meta/tables/{table_id}/hooks"))
        .with_json(&body)?
        .exec()
        .await?;

    log::log(log::Message {
        id: "noco-hook-created",
        env,
        msg: &format!("Created Noco hook `{}` on table `{}`", title, table_id),
    });

    Ok(())
}
