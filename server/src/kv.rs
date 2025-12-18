use secrecy::ExposeSecret;
use worker::kv::{KvError, KvStore};

use crate::{
    env::{Config, EnvId, EnvName},
    noco::{About, Announcement, ApiToken, Event, Info, Page, TableInfo},
};

fn wrap_kv_err(err: KvError) -> anyhow::Error {
    anyhow::Error::msg(err.to_string())
}

// This random ID forms part of the app URL gives to attendees. We use a random ID instead of the
// environment name for two reasons:
// 1. The environment name isn't meant to change; it's used to identify a lot of resources in the
//    infrastructure. Therefore, it shouldn't be user-facing. If we offered vanity URLs to cons, it
//    would be reasonable for them to expect we're able to change it for them. By decoupling them,
//    we can change one without the other.
// 2. Some cons (e.g. adult-only cons) might not want their app URL to be guessable; they might not
//    want non-attendees to be able to easily access the schedule.
fn env_id_key(env_name: &EnvName) -> String {
    format!("env:{env_name}:id")
}

// We need to map environment ID to environment name because the client app will be making requests
// to this service by the environment ID.
fn id_env_key(env_id: &EnvId) -> String {
    format!("id:{env_id}:env")
}

// The NocoDB API token for the environment. This is used to authenticate with the NocoDB API.
fn api_token_key(env_name: &EnvName) -> String {
    format!("env:{env_name}:api-token")
}

// The cached IDs of the known tables in NocoDB.
fn tables_key(env_name: &EnvName) -> String {
    format!("env:{env_name}:tables")
}

// Environment-specific config values.
fn env_config_key(env_name: &EnvName) -> String {
    format!("env:{env_name}:config")
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
cache_key_fn!(about_cache_key, "about");

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
put_cache_fn!(put_cached_about, about_cache_key, &About);

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
get_cache_fn!(get_cached_about, about_cache_key, About);

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
