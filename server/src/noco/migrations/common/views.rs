use std::fmt;

use serde::Deserialize;
use serde_json::json;
use worker::{Method, console_log};

use crate::noco::Client;

use super::{RefSetter, TableId, ViewId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewType {
    Form,
}

impl ViewType {
    fn endpoint(self) -> &'static str {
        match self {
            ViewType::Form => "forms",
        }
    }

    pub fn code(self) -> u32 {
        match self {
            ViewType::Form => 1,
        }
    }
}

pub struct ViewRequest<'a> {
    pub body: serde_json::Value,
    pub table_id: TableId,
    pub kind: ViewType,
    pub table_ref: RefSetter<'a, ViewId>,
}

impl fmt::Debug for ViewRequest<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ViewRequest")
            .field("body", &self.body)
            .finish_non_exhaustive()
    }
}

pub async fn create_views(client: &Client, requests: Vec<ViewRequest<'_>>) -> anyhow::Result<()> {
    #[derive(Debug, Deserialize)]
    struct PostViewResponse {
        id: ViewId,
    }

    for request in requests {
        let view_id = client
            .build_request_v2(
                Method::Post,
                &format!(
                    "/meta/tables/{}/{}",
                    request.table_id,
                    request.kind.endpoint()
                ),
            )
            .with_json(&request.body)?
            .fetch::<PostViewResponse>()
            .await?
            .id;

        let view_name = request
            .body
            .as_object()
            .and_then(|obj| obj.get("title"))
            .and_then(|title| title.as_str())
            .unwrap_or("Unknown");

        console_log!(
            "Created Noco view `{}` of type {:?} with ID `{}`",
            view_name,
            request.kind,
            view_id
        );

        (request.table_ref)(view_id.clone());
    }

    Ok(())
}

pub async fn lock_views(client: &Client, views: Vec<ViewId>) -> anyhow::Result<()> {
    for view_id in views {
        client
            .build_request_v2(Method::Patch, &format!("/meta/views/{view_id}"))
            .with_json(&json!({
                "lock_type": "locked",
            }))?
            .exec()
            .await?;

        console_log!("Locked Noco view with ID `{}`", view_id,);
    }

    Ok(())
}
