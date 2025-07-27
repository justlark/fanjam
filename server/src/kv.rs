use secrecy::ExposeSecret;
use worker::kv::{KvError, KvStore};

use crate::{
    config,
    env::{EnvId, EnvName},
    noco::{self, ApiToken, BaseId, Event, Info, TableInfo},
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

// The NocoDB base ID for the environment. Each NocoDB instance contains a single base.
fn base_id_key(env_name: &EnvName) -> String {
    format!("env:{env_name}:base-id")
}

// The cached ID of a table in NocoDB.
fn tables_key(env_name: &EnvName) -> String {
    format!("env:{env_name}:tables")
}

// The current schema migration number of an environment. This is how we know where to start when
// applying migrations, so we don't accidentally apply the same migration twice.
fn migration_version_key(env_name: &EnvName) -> String {
    format!("env:{env_name}:migration")
}

// We cache responses from the upstream NocoDB server to reduce the load on it.
fn events_cache_key(env_name: &EnvName) -> String {
    format!("env:{env_name}:cache:events")
}

fn info_cache_key(env_name: &EnvName) -> String {
    format!("env:{env_name}:cache:info")
}

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
    if let Some(env_name) = maybe_env_name {
        if get_env_id(kv, &env_name).await?.as_ref() == Some(env_id) {
            return Ok(Some(env_name));
        }
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
pub async fn delete_base_id(kv: &KvStore, env_name: &EnvName) -> anyhow::Result<()> {
    kv.delete(&base_id_key(env_name))
        .await
        .map_err(wrap_kv_err)?;

    Ok(())
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
pub async fn put_migration_version(
    kv: &KvStore,
    env_name: &EnvName,
    version: noco::Version,
) -> anyhow::Result<()> {
    kv.put(&migration_version_key(env_name), version.to_string())
        .map_err(wrap_kv_err)?
        .execute()
        .await
        .map_err(wrap_kv_err)?;

    Ok(())
}

#[worker::send]
pub async fn get_migration_version(
    kv: &KvStore,
    env_name: &EnvName,
) -> anyhow::Result<Option<noco::Version>> {
    kv.get(&migration_version_key(env_name))
        .text()
        .await
        .map_err(wrap_kv_err)?
        .map(|s| s.parse())
        .transpose()
}

#[worker::send]
pub async fn delete_migration_version(kv: &KvStore, env_name: &EnvName) -> anyhow::Result<()> {
    kv.delete(&migration_version_key(env_name))
        .await
        .map_err(wrap_kv_err)?;

    Ok(())
}

#[worker::send]
pub async fn put_cached_events(
    kv: &KvStore,
    env_name: &EnvName,
    events: &[Event],
) -> anyhow::Result<()> {
    let ttl = config::noco_cache_ttl();

    if ttl.is_zero() {
        // If the TTL is zero, we have nothing to cache and can just return silently.
        return Ok(());
    }

    kv.put(&events_cache_key(env_name), events)
        .map_err(wrap_kv_err)?
        .expiration_ttl(ttl.as_secs())
        .execute()
        .await
        .map_err(wrap_kv_err)?;

    Ok(())
}

#[worker::send]
pub async fn get_cached_events(
    kv: &KvStore,
    env_name: &EnvName,
) -> anyhow::Result<Option<Vec<Event>>> {
    kv.get(&events_cache_key(env_name))
        .json::<Vec<Event>>()
        .await
        .map_err(wrap_kv_err)
}

#[worker::send]
pub async fn put_cached_info(kv: &KvStore, env_name: &EnvName, info: &Info) -> anyhow::Result<()> {
    let ttl = config::noco_cache_ttl();

    if ttl.is_zero() {
        // If the TTL is zero, we have nothing to cache and can just return silently.
        return Ok(());
    }

    kv.put(&info_cache_key(env_name), info)
        .map_err(wrap_kv_err)?
        .expiration_ttl(ttl.as_secs())
        .execute()
        .await
        .map_err(wrap_kv_err)?;

    Ok(())
}

#[worker::send]
pub async fn get_cached_info(kv: &KvStore, env_name: &EnvName) -> anyhow::Result<Option<Info>> {
    kv.get(&info_cache_key(env_name))
        .json::<Info>()
        .await
        .map_err(wrap_kv_err)
}
