use secrecy::ExposeSecret;
use serde::{Deserialize, Serialize};
use worker::kv::{KvError, KvStore};

use crate::{
    api::Alias,
    env::{Config, EnvDomain, EnvId, EnvName},
    noco::{Announcement, ApiToken, BaseId, Event, File, Info, Page, TableInfo},
    push,
};

fn wrap_kv_err(err: KvError) -> anyhow::Error {
    anyhow::Error::msg(err.to_string())
}

// This ID forms part of the app URL gives to attendees. We use an ID distinct from the environment
// name because the environment name isn't meant to change; it's used to identify a lot of
// resources in the infrastructure. Therefore, it shouldn't be user-facing. It is reasonable for
// cons to expect we're able to change the URL for them. By decoupling the environment name from
// the environment ID, we can change one without the other.
fn env_id_key(env_name: &EnvName) -> String {
    format!("env:{env_name}:id")
}

// We need to map environment ID to environment name because the client app will be making requests
// to this service by the environment ID.
fn id_env_key(env_id: &EnvId) -> String {
    format!("id:{env_id}:env")
}

const ALIAS_KEY_PREFIX: &str = "alias:";
const ALIAS_ID_KEY_SUFFIX: &str = ":id";

// If we ever change the environment ID, we may want to keep the old ID as an alias which redirects
// to the new one.
fn alias_id_key(env_id: &EnvId) -> String {
    format!("{ALIAS_KEY_PREFIX}{env_id}{ALIAS_ID_KEY_SUFFIX}")
}

// The custom domain for an environment. Absent for environments without a custom domain.
fn env_domain_key(env_name: &EnvName) -> String {
    format!("env:{env_name}:domain")
}

// The environment with a given custom domain.
fn domain_env_key(domain: &EnvDomain) -> String {
    format!("domain:{domain}:env")
}

// The NocoDB API token for the environment. This is used to authenticate with the NocoDB API.
fn api_token_key(env_name: &EnvName) -> String {
    format!("env:{env_name}:api-token")
}

// The cached IDs of the known tables in NocoDB.
fn tables_key(env_name: &EnvName) -> String {
    format!("env:{env_name}:tables")
}

// The cached ID of the current NocoDB base for this environment.
//
// The Postgres database is the source of truth for this, but we cache it in KV to avoid needing to
// open a new Postgres connection per incoming request. This cache must be invalidated whenever the
// base changes, such as when we restore from a database backup.
fn base_id_key(env_name: &EnvName) -> String {
    format!("env:{env_name}:base-id")
}

// Environment-specific config values.
fn env_config_key(env_name: &EnvName) -> String {
    format!("env:{env_name}:config")
}

// Per-environment push notification subscriptions. The suffix is a stable
// hash of the subscription endpoint URL (see `push::Subscription::id`), so
// re-POSTing the same subscription is idempotent and listing all
// subscriptions for an environment is a single `prefix` scan.
fn subscription_key_prefix(env_name: &EnvName) -> String {
    format!("env:{env_name}:subscription:")
}

fn subscription_key(env_name: &EnvName, subscription_id: &str) -> String {
    format!("{}{}", subscription_key_prefix(env_name), subscription_id)
}

fn cache_key_prefix(env_name: &EnvName) -> String {
    format!("env:{env_name}:cache:")
}

macro_rules! cache_key_fn {
    ($name:ident, $key:expr) => {
        pub fn $name(env_name: &EnvName) -> String {
            format!("{}{}", cache_key_prefix(env_name), $key)
        }
    };
}

cache_key_fn!(events_cache_key, "events");
cache_key_fn!(info_cache_key, "info");
cache_key_fn!(pages_cache_key, "pages");
cache_key_fn!(announcements_cache_key, "announcements");
cache_key_fn!(files_cache_key, "files");

#[worker::send]
pub async fn put_id_env(kv: &KvStore, env_id: &EnvId, env_name: &EnvName) -> anyhow::Result<()> {
    kv.put(&id_env_key(env_id), env_name)
        .map_err(wrap_kv_err)?
        .execute()
        .await
        .map_err(wrap_kv_err)?;

    Ok(())
}

#[worker::send]
pub async fn get_id_env(kv: &KvStore, env_id: &EnvId) -> anyhow::Result<Option<EnvName>> {
    let maybe_env_name = kv
        .get(&id_env_key(env_id))
        .text()
        .await
        .map_err(wrap_kv_err)?
        .map(EnvName::from);

    // Only the latest event ID should work. This allows us to invalidate old app links.
    if let Some(env_name) = maybe_env_name
        && get_env_id(kv, &env_name).await?.as_ref() == Some(env_id)
    {
        return Ok(Some(env_name));
    }

    return Ok(None);
}

#[worker::send]
pub async fn put_alias_id(kv: &KvStore, from: &EnvId, to: &EnvId) -> anyhow::Result<()> {
    kv.put(&alias_id_key(from), to)
        .map_err(wrap_kv_err)?
        .execute()
        .await
        .map_err(wrap_kv_err)?;

    Ok(())
}

#[worker::send]
pub async fn get_alias_id(kv: &KvStore, alias: &EnvId) -> anyhow::Result<Option<EnvId>> {
    Ok(kv
        .get(&alias_id_key(alias))
        .text()
        .await
        .map_err(wrap_kv_err)?
        .map(EnvId::from))
}

#[worker::send]
pub async fn delete_alias_id(kv: &KvStore, alias: &EnvId) -> anyhow::Result<()> {
    kv.delete(&alias_id_key(alias)).await.map_err(wrap_kv_err)?;

    Ok(())
}

#[worker::send]
pub async fn list_aliases(kv: &KvStore) -> anyhow::Result<Vec<Alias>> {
    // The default and maximum number of keys this will return is 1000, which is more than plenty
    // that we don't have to worry about pagination.
    let aliases = kv
        .list()
        .prefix(ALIAS_KEY_PREFIX.to_string())
        .execute()
        .await
        .map_err(wrap_kv_err)?
        .keys
        .into_iter()
        .filter_map(|key| {
            key.name
                .strip_prefix(ALIAS_KEY_PREFIX)
                .map(ToString::to_string)
        })
        .filter_map(|key| {
            key.strip_suffix(ALIAS_ID_KEY_SUFFIX)
                .map(ToString::to_string)
        })
        .collect::<Vec<_>>();

    let mut pairs = Vec::with_capacity(aliases.len());

    for alias in aliases {
        if let Some(target) = get_alias_id(kv, &EnvId::from(alias.clone())).await? {
            pairs.push(Alias {
                alias,
                target: target.to_string(),
            });
        }
    }

    Ok(pairs)
}

#[worker::send]
pub async fn put_env_id(kv: &KvStore, env_name: &EnvName, env_id: &EnvId) -> anyhow::Result<()> {
    kv.put(&env_id_key(env_name), env_id)
        .map_err(wrap_kv_err)?
        .execute()
        .await
        .map_err(wrap_kv_err)?;

    Ok(())
}

#[worker::send]
pub async fn get_env_id(kv: &KvStore, env_name: &EnvName) -> anyhow::Result<Option<EnvId>> {
    Ok(kv
        .get(&env_id_key(env_name))
        .text()
        .await
        .map_err(wrap_kv_err)?
        .map(EnvId::from))
}

#[worker::send]
pub async fn put_env_domain(
    kv: &KvStore,
    env_name: &EnvName,
    domain: &EnvDomain,
) -> anyhow::Result<()> {
    kv.put(&env_domain_key(env_name), domain.as_str())
        .map_err(wrap_kv_err)?
        .execute()
        .await
        .map_err(wrap_kv_err)?;

    Ok(())
}

#[worker::send]
pub async fn get_env_domain(kv: &KvStore, env_name: &EnvName) -> anyhow::Result<Option<EnvDomain>> {
    let raw = kv
        .get(&env_domain_key(env_name))
        .text()
        .await
        .map_err(wrap_kv_err)?;

    raw.map(EnvDomain::try_from).transpose()
}

#[worker::send]
pub async fn delete_env_domain(kv: &KvStore, env_name: &EnvName) -> anyhow::Result<()> {
    kv.delete(&env_domain_key(env_name))
        .await
        .map_err(wrap_kv_err)?;

    Ok(())
}

#[worker::send]
pub async fn put_domain_env(
    kv: &KvStore,
    domain: &EnvDomain,
    env_name: &EnvName,
) -> anyhow::Result<()> {
    kv.put(&domain_env_key(domain), env_name.to_string())
        .map_err(wrap_kv_err)?
        .execute()
        .await
        .map_err(wrap_kv_err)?;

    Ok(())
}

#[worker::send]
pub async fn get_domain_env(kv: &KvStore, domain: &EnvDomain) -> anyhow::Result<Option<EnvName>> {
    Ok(kv
        .get(&domain_env_key(domain))
        .text()
        .await
        .map_err(wrap_kv_err)?
        .map(EnvName::from))
}

#[worker::send]
pub async fn delete_domain_env(kv: &KvStore, domain: &EnvDomain) -> anyhow::Result<()> {
    kv.delete(&domain_env_key(domain))
        .await
        .map_err(wrap_kv_err)?;

    Ok(())
}

#[worker::send]
pub async fn put_api_token(
    kv: &KvStore,
    env_name: &EnvName,
    api_token: ApiToken,
) -> anyhow::Result<()> {
    kv.put(&api_token_key(env_name), api_token.expose_secret())
        .map_err(wrap_kv_err)?
        .execute()
        .await
        .map_err(wrap_kv_err)?;

    Ok(())
}

#[worker::send]
pub async fn get_api_token(kv: &KvStore, env_name: &EnvName) -> anyhow::Result<Option<ApiToken>> {
    Ok(kv
        .get(&api_token_key(env_name))
        .text()
        .await
        .map_err(wrap_kv_err)?
        .map(ApiToken::from))
}

#[worker::send]
pub async fn put_tables(
    kv: &KvStore,
    env_name: &EnvName,
    tables: &[TableInfo],
) -> anyhow::Result<()> {
    kv.put(&tables_key(env_name), tables)
        .map_err(wrap_kv_err)?
        .execute()
        .await
        .map_err(wrap_kv_err)?;

    Ok(())
}

#[worker::send]
pub async fn get_tables(kv: &KvStore, env_name: &EnvName) -> anyhow::Result<Vec<TableInfo>> {
    Ok(kv
        .get(&tables_key(env_name))
        .json::<Vec<TableInfo>>()
        .await
        .map_err(wrap_kv_err)?
        .unwrap_or_default())
}

#[worker::send]
async fn delete_tables(kv: &KvStore, env_name: &EnvName) -> anyhow::Result<()> {
    kv.delete(&tables_key(env_name))
        .await
        .map_err(wrap_kv_err)?;

    Ok(())
}

#[worker::send]
pub async fn put_base_id(kv: &KvStore, env_name: &EnvName, base_id: &BaseId) -> anyhow::Result<()> {
    kv.put(&base_id_key(env_name), base_id.to_string())
        .map_err(wrap_kv_err)?
        .execute()
        .await
        .map_err(wrap_kv_err)?;

    Ok(())
}

#[worker::send]
pub async fn get_base_id(kv: &KvStore, env_name: &EnvName) -> anyhow::Result<Option<BaseId>> {
    Ok(kv
        .get(&base_id_key(env_name))
        .text()
        .await
        .map_err(wrap_kv_err)?
        .map(BaseId::from))
}

#[worker::send]
async fn delete_base_id(kv: &KvStore, env_name: &EnvName) -> anyhow::Result<()> {
    kv.delete(&base_id_key(env_name))
        .await
        .map_err(wrap_kv_err)?;

    Ok(())
}

macro_rules! put_cache_fn {
    ($name:ident, $key_fn:expr, $type:ty) => {
        #[worker::send]
        pub async fn $name(kv: &KvStore, env_name: &EnvName, value: $type) -> anyhow::Result<()> {
            kv.put(&$key_fn(env_name), value)
                .map_err(wrap_kv_err)?
                .execute()
                .await
                .map_err(wrap_kv_err)?;

            Ok(())
        }
    };
}

put_cache_fn!(put_cached_events, events_cache_key, &[Event]);
put_cache_fn!(put_cached_info, info_cache_key, &Info);
put_cache_fn!(put_cached_pages, pages_cache_key, &[Page]);
put_cache_fn!(
    put_cached_announcements,
    announcements_cache_key,
    &[Announcement]
);
put_cache_fn!(put_cached_files, files_cache_key, &[File]);

macro_rules! get_cache_fn {
    ($name:ident, $key_fn:expr, $type:ty) => {
        #[worker::send]
        pub async fn $name(kv: &KvStore, env_name: &EnvName) -> anyhow::Result<Option<$type>> {
            let key = $key_fn(env_name);

            kv.get(&key).json::<$type>().await.map_err(wrap_kv_err)
        }
    };
}

get_cache_fn!(get_cached_events, events_cache_key, Vec<Event>);
get_cache_fn!(get_cached_info, info_cache_key, Info);
get_cache_fn!(get_cached_pages, pages_cache_key, Vec<Page>);
get_cache_fn!(
    get_cached_announcements,
    announcements_cache_key,
    Vec<Announcement>
);
get_cache_fn!(get_cached_files, files_cache_key, Vec<File>);

#[worker::send]
pub async fn delete_cache(kv: &KvStore, env_name: &EnvName) -> anyhow::Result<()> {
    // The default and maximum number of keys this will return is 1000, which is more than plenty
    // that we don't have to worry about pagination.
    let cache_keys = kv
        .list()
        .prefix(cache_key_prefix(env_name))
        .execute()
        .await
        .map_err(wrap_kv_err)?
        .keys;

    for key in cache_keys {
        kv.delete(&key.name).await.map_err(wrap_kv_err)?;
    }

    delete_tables(kv, env_name).await?;
    delete_base_id(kv, env_name).await?;

    Ok(())
}

#[worker::send]
pub async fn put_env_config(
    kv: &KvStore,
    env_name: &EnvName,
    config: &Config,
) -> anyhow::Result<()> {
    kv.put(&env_config_key(env_name), config)
        .map_err(wrap_kv_err)?
        .execute()
        .await
        .map_err(wrap_kv_err)
}

#[worker::send]
pub async fn get_env_config(kv: &KvStore, env_name: &EnvName) -> anyhow::Result<Config> {
    Ok(kv
        .get(&env_config_key(env_name))
        .json::<Config>()
        .await
        .map_err(wrap_kv_err)?
        .unwrap_or_default())
}

/// A push subscription as stored in KV: the wire-format subscription the
/// client POSTed, plus a server-assigned `created_at` timestamp for ageing
/// decisions (currently informational only).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredSubscription {
    #[serde(flatten)]
    pub subscription: push::Subscription,
    pub created_at: i64,
}

#[worker::send]
pub async fn put_subscription(
    kv: &KvStore,
    env_name: &EnvName,
    subscription: &push::Subscription,
) -> anyhow::Result<()> {
    let stored = StoredSubscription {
        subscription: subscription.clone(),
        created_at: chrono::Utc::now().timestamp(),
    };
    let key = subscription_key(env_name, &subscription.id());

    kv.put(&key, &stored)
        .map_err(wrap_kv_err)?
        .execute()
        .await
        .map_err(wrap_kv_err)?;

    Ok(())
}

#[worker::send]
pub async fn delete_subscription(
    kv: &KvStore,
    env_name: &EnvName,
    subscription_id: &str,
) -> anyhow::Result<()> {
    kv.delete(&subscription_key(env_name, subscription_id))
        .await
        .map_err(wrap_kv_err)?;

    Ok(())
}

/// List every subscription stored under this environment, paginating through
/// KV's 1000-keys-per-page limit. The webhook fan-out path (slice 3) iterates
/// this list and sends a push to each subscription; a popular convention
/// could plausibly cross 1000, so we handle the cursor properly even though
/// every other `kv::list` caller in this codebase stops at one page.
#[worker::send]
#[allow(dead_code)] // consumed by the webhook fan-out in slice 3
pub async fn list_subscriptions(
    kv: &KvStore,
    env_name: &EnvName,
) -> anyhow::Result<Vec<StoredSubscription>> {
    let prefix = subscription_key_prefix(env_name);
    let mut cursor: Option<String> = None;
    let mut out = Vec::new();

    loop {
        let mut list = kv.list().prefix(prefix.clone());
        if let Some(c) = cursor.as_deref() {
            list = list.cursor(c.to_string());
        }
        let page = list.execute().await.map_err(wrap_kv_err)?;

        for key in &page.keys {
            if let Some(stored) = kv
                .get(&key.name)
                .json::<StoredSubscription>()
                .await
                .map_err(wrap_kv_err)?
            {
                out.push(stored);
            }
            // If the key vanished between `list` and `get`, just skip it —
            // a concurrent DELETE on the same subscription is benign.
        }

        match page.cursor {
            Some(c) if !page.list_complete => cursor = Some(c),
            _ => break,
        }
    }

    Ok(out)
}
