use crate::noco::{
    BaseId, Client, TableIds, Version, list_tables,
    migrations::{
        common::{
            self, MigrationContext, PUSH_NOTIFICATIONS_HOOK_TITLE, create_url_hook, delete_hook,
            list_hooks,
        },
        n10,
    },
};

// Replace the push notifications hook with one that calls an endpoint keyed by the environment name.
//
// The hook n05 installed called an endpoint keyed by the env ID. Env IDs change when the app slug
// changes, and the old ID stops resolving, so the hook would silently start failing. The env name is
// stable for the life of the environment.
//
// This also installs the hook on bases where n05 skipped it, because the environment didn't have an
// env ID yet at the time.
pub struct Migration<'a> {
    client: &'a Client,
    ctx: &'a MigrationContext,
}

impl<'a> common::Migration<'a> for Migration<'a> {
    const INDEX: Version = n10::Migration::INDEX.next();

    fn new(client: &'a Client, ctx: &'a MigrationContext) -> Self {
        Self { client, ctx }
    }

    async fn migrate(&self, base_id: BaseId) -> anyhow::Result<()> {
        // Check this before touching anything. Every stage is provisioned with this secret, so if
        // it's missing, the worker is misconfigured. Failing rolls the migration back so it can be
        // retried once the secret is set, rather than recording it as done without a hook.
        let Some(token) = self.ctx.noco_webhook_token.as_ref() else {
            anyhow::bail!(
                "Cannot install the `{PUSH_NOTIFICATIONS_HOOK_TITLE}` hook: NOCO_WEBHOOK_TOKEN is \
                 not configured for this worker."
            );
        };

        let tables = TableIds::try_from(list_tables(self.client, &base_id).await?)?;

        let existing_hooks = list_hooks(self.client, &tables.announcements).await?;

        for hook in existing_hooks
            .iter()
            .filter(|hook| hook.title.as_deref() == Some(PUSH_NOTIFICATIONS_HOOK_TITLE))
        {
            delete_hook(self.client, &hook.id, (&self.ctx.env_name).into()).await?;
        }

        let url = format!(
            "https://{}/env/{}/hooks/announcement-created",
            self.ctx.api_domain, self.ctx.env_name,
        );

        create_url_hook(
            self.client,
            &tables.announcements,
            PUSH_NOTIFICATIONS_HOOK_TITLE,
            &url,
            token,
            (&self.ctx.env_name).into(),
        )
        .await?;

        Ok(())
    }
}
