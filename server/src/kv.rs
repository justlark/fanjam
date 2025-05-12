use secrecy::ExposeSecret;
use worker::kv::{KvError, KvStore};

use crate::{
    env::{EnvId, new_env_id},
    noco::{ApiToken, BaseId},
};

fn wrap_kv_err(err: KvError) -> anyhow::Error {
    anyhow::Error::msg(err.to_string())
}

fn env_id_key(env_name: &str) -> String {
    format!("env-id:{}", env_name)
}

fn api_token_key(env_name: &str) -> String {
    format!("noco-api-token:{}", env_name)
}

fn base_id_key(env_name: &str) -> String {
    format!("noco-base-id:{}", env_name)
}

#[worker::send]
pub async fn put_env_id(kv: &KvStore, env_name: &str) -> anyhow::Result<EnvId> {
    let env_id = new_env_id();

    kv.put(&env_id_key(env_name), &env_id)
        .map_err(wrap_kv_err)?
        .execute()
        .await
        .map_err(wrap_kv_err)?;

    Ok(env_id)
}

#[worker::send]
pub async fn get_env_id(kv: &KvStore, env_name: &str) -> anyhow::Result<Option<EnvId>> {
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
    env_name: &str,
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
pub async fn get_api_token(kv: &KvStore, env_name: &str) -> anyhow::Result<Option<ApiToken>> {
    Ok(kv
        .get(&api_token_key(env_name))
        .text()
        .await
        .map_err(wrap_kv_err)?
        .map(ApiToken::from))
}

#[worker::send]
pub async fn put_base_id(kv: &KvStore, env_name: &str, base_id: &BaseId) -> anyhow::Result<()> {
    kv.put(&base_id_key(env_name), base_id.to_string())
        .map_err(wrap_kv_err)?
        .execute()
        .await
        .map_err(wrap_kv_err)?;

    Ok(())
}

#[worker::send]
pub async fn get_base_id(kv: &KvStore, env_name: &str) -> anyhow::Result<Option<BaseId>> {
    Ok(kv
        .get(&base_id_key(env_name))
        .text()
        .await
        .map_err(wrap_kv_err)?
        .map(BaseId::from))
}
