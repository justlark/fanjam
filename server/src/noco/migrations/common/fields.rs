use std::fmt;

use reqwest::Method;
use serde::Deserialize;
use worker::console_log;

use crate::noco::{Client, client::check_status};

use super::{FieldId, RefSetter, TableId};

pub struct FieldRequest<'a> {
    pub table_id: &'a TableId,
    pub field_ref: RefSetter<'a, FieldId>,
    pub body: serde_json::Value,
}

impl fmt::Debug for FieldRequest<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FieldRequest")
            .field("table_id", &self.table_id)
            .field("body", &self.body)
            .finish_non_exhaustive()
    }
}

pub async fn create_fields(client: &Client, requests: Vec<FieldRequest<'_>>) -> anyhow::Result<()> {
    #[derive(Debug, Deserialize)]
    struct PostFieldResponse {
        id: FieldId,
    }

    for request in requests {
        let resp = client
            .build_request_v3(
                Method::POST,
                &format!("/meta/tables/{}/fields", request.table_id),
            )
            .json(&request.body)
            .send()
            .await?;

        let field_id = check_status(resp)
            .await?
            .json::<PostFieldResponse>()
            .await?
            .id;

        let field_name = request
            .body
            .as_object()
            .and_then(|obj| obj.get("title"))
            .and_then(|title| title.as_str())
            .unwrap_or("Unknown");

        (request.field_ref)(field_id.clone());

        console_log!(
            "Created Noco field `{}` with ID `{}` on table `{}`",
            field_name,
            field_id,
            request.table_id,
        );
    }

    Ok(())
}
