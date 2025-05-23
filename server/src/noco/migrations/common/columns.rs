use std::fmt;

use reqwest::Method;
use serde::Deserialize;
use worker::console_log;

use crate::{http::check_status, noco::Client};

use super::{ColumnId, RefSetter, TableId};

pub struct ColumnRequest<'a> {
    pub table_id: &'a TableId,
    pub column_ref: RefSetter<'a, ColumnId>,
    pub body: serde_json::Value,
}

impl fmt::Debug for ColumnRequest<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FieldRequest")
            .field("table_id", &self.table_id)
            .field("body", &self.body)
            .finish_non_exhaustive()
    }
}

pub async fn create_columns(
    client: &Client,
    requests: Vec<ColumnRequest<'_>>,
) -> anyhow::Result<()> {
    #[derive(Debug, Deserialize)]
    struct PostColumnResponse {
        id: ColumnId,
    }

    for request in requests {
        let resp = client
            .build_request_v2(
                Method::POST,
                &format!("/meta/tables/{}/columns", request.table_id),
            )
            .json(&request.body)
            .send()
            .await?;

        let column_id = check_status(resp)
            .await?
            .json::<PostColumnResponse>()
            .await?
            .id;

        let column_name = request
            .body
            .as_object()
            .and_then(|obj| obj.get("title"))
            .and_then(|title| title.as_str())
            .unwrap_or("Unknown");

        (request.column_ref)(column_id.clone());

        console_log!(
            "Created Noco column `{}` with ID `{}` on table `{}`",
            column_name,
            column_id,
            request.table_id,
        );
    }

    Ok(())
}
