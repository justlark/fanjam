use std::fmt;

use reqwest::Method;
use serde::Deserialize;
use worker::console_log;

use crate::noco::{Client, client::check_status};

use super::{BaseId, RefSetter, TableId};

pub struct TableRequest<'a> {
    pub body: serde_json::Value,
    pub table_ref: RefSetter<'a, TableId>,
}

impl fmt::Debug for TableRequest<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TableRequest")
            .field("body", &self.body)
            .finish_non_exhaustive()
    }
}

pub async fn create_tables(
    client: &Client,
    base_id: &BaseId,
    requests: Vec<TableRequest<'_>>,
) -> anyhow::Result<()> {
    #[derive(Debug, Deserialize)]
    struct PostTableResponse {
        id: TableId,
    }

    for request in requests {
        let resp = client
            .build_request_v2(Method::POST, &format!("/meta/bases/{}/tables", base_id))
            .json(&request.body)
            .send()
            .await?;

        let table_id = check_status(resp)
            .await?
            .json::<PostTableResponse>()
            .await?
            .id;

        let table_name = request
            .body
            .as_object()
            .and_then(|obj| obj.get("title"))
            .and_then(|title| title.as_str())
            .unwrap_or("Unknown");

        console_log!("Created Noco table `{}` with ID `{}`", table_name, table_id);

        (request.table_ref)(table_id.clone());
    }

    Ok(())
}
