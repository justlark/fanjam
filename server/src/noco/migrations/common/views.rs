use reqwest::Method;
use serde_json::json;
use worker::console_log;

use crate::noco::{Client, client::check_status};

use super::ViewId;

pub async fn lock_views(client: &Client, views: Vec<ViewId>) -> anyhow::Result<()> {
    for view_id in views {
        let resp = client
            .build_request_v2(Method::PATCH, &format!("/meta/views/{}", view_id))
            .json(&json!({
                "lock_type": "locked",
            }))
            .send()
            .await?;

        check_status(resp).await?;

        console_log!("Locked Noco view with ID `{}`", view_id,);
    }

    Ok(())
}
