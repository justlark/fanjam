use secrecy::ExposeSecret;
use worker::kv::{KvError, KvStore};

use crate::{
    env::{EnvId, EnvName},
    noco::{ApiToken, BaseId},
};

fn wrap_kv_err(err: KvError) -> anyhow::Error {
    anyhow::Error::msg(err.to_string())
}

fn id_env_key(env_id: &EnvId) -> String {
    format!("id:{}:env", env_id)
}

fn env_id_key(env_name: &EnvName) -> String {
    format!("env:{}:id", env_name)
}

fn api_token_key(env_name: &EnvName) -> String {
    format!("env:{}:api-token", env_name)
}

fn base_id_key(env_name: &EnvName) -> String {
    format!("env:{}:base-id", env_name)
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
