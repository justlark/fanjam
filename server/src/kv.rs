use secrecy::ExposeSecret;
use worker::kv::{KvError, KvStore};

use crate::{env::EnvId, noco::ApiToken};

fn wrap_kv_err(err: KvError) -> anyhow::Error {
    anyhow::Error::msg(err.to_string())
}

fn api_token_key(env: &EnvId) -> String {
    format!("noco-api-token:{}", env)
}

#[worker::send]
pub async fn put_api_token(
    kv: &KvStore,
    env_id: &EnvId,
    api_token: ApiToken,
) -> anyhow::Result<()> {
    kv.put(&api_token_key(env_id), api_token.expose_secret())
        .map_err(wrap_kv_err)?
        .execute()
        .await
        .map_err(wrap_kv_err)?;

    Ok(())
}
