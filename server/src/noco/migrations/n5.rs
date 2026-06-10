use base64::prelude::*;
use secrecy::ExposeSecret;
use serde::Deserialize;
use serde_json::json;
use worker::{Method, console_log, console_warn};

use crate::noco::{
    BaseId, Client, TableIds, Version, list_tables,
    migrations::{
        common::{self, MigrationContext},
        n4,
    },
};

// Title we put on the hook record. Used as the idempotency key on re-runs
// — if a hook with this title already exists on the table, we leave it alone.
const HOOK_TITLE: &str = "FanJam push notifications";

pub struct Migration<'a> {
    client: &'a Client,
    ctx: &'a MigrationContext,
}

#[derive(Debug, Deserialize)]
struct HookSummary {
    title: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ListHooksResponse {
    list: Vec<HookSummary>,
}

impl<'a> Migration<'a> {
    async fn hook_already_installed(
        &self,
        announcements_table: &common::TableId,
    ) -> anyhow::Result<bool> {
        let response = self
            .client
            .build_request_v2(
                Method::Get,
                &format!("/meta/tables/{announcements_table}/hooks"),
            )
            .fetch::<ListHooksResponse>()
            .await?;

        Ok(response
            .list
            .iter()
            .any(|hook| hook.title.as_deref() == Some(HOOK_TITLE)))
    }

    async fn install_hook(&self, announcements_table: &common::TableId) -> anyhow::Result<()> {
        // The webhook path is per-environment because the server multiplexes
        // every environment behind one shared origin; the receiver routes by
        // env_id back to the right KV namespace.
        let Some(env_id) = self.ctx.env_id.as_ref() else {
            console_warn!(
                "Skipping `{HOOK_TITLE}` hook: this environment has no env_id assigned yet.",
            );
            return Ok(());
        };
        let Some(token) = self.ctx.noco_webhook_token.as_ref() else {
            console_warn!(
                "Skipping `{HOOK_TITLE}` hook: NOCO_WEBHOOK_TOKEN is not configured for this worker.",
            );
            return Ok(());
        };

        let url = format!(
            "https://{}/apps/{env_id}/hooks/announcement-created",
            self.ctx.api_domain,
        );

        let bearer = BASE64_STANDARD.encode(token.expose_secret());
        // The `{{ json data }}` Handlebars template renders the inserted rows when the hook fires.
        let body = json!({
            "title": HOOK_TITLE,
            "event": "after",
            "operation": "insert",
            "version": "v2",
            "active": true,
            "notification": {
                "type": "URL",
                "payload": {
                    "method": "POST",
                    "path": url,
                    "headers": [
                        { "name": "Authorization", "value": format!("Bearer {bearer}") },
                        { "name": "Content-Type", "value": "application/json" },
                    ],
                    "body": "{{ json data }}",
                },
            },
        });

        self.client
            .build_request_v2(
                Method::Post,
                &format!("/meta/tables/{announcements_table}/hooks"),
            )
            .with_json(&body)?
            .exec()
            .await?;

        console_log!("Installed NocoDB hook `{HOOK_TITLE}` on the announcements table");

        Ok(())
    }
}

impl<'a> common::Migration<'a> for Migration<'a> {
    const INDEX: Version = n4::Migration::INDEX.next();

    fn new(client: &'a Client, ctx: &'a MigrationContext) -> Self {
        Self { client, ctx }
    }

    async fn migrate(&self, base_id: BaseId) -> anyhow::Result<()> {
        let tables = TableIds::try_from(list_tables(self.client, &base_id).await?)?;

        // Idempotent: re-running the migration after the hook is in place is
        // a no-op. This matters because the user can reset migration state
        // and replay.
        if self.hook_already_installed(&tables.announcements).await? {
            console_log!("NocoDB hook `{HOOK_TITLE}` already installed; skipping.");
            return Ok(());
        }

        self.install_hook(&tables.announcements).await?;

        Ok(())
    }
}
