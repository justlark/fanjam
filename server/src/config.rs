use std::sync::OnceLock;

use worker::Env;

use crate::noco::ApiToken;

#[derive(Debug)]
struct Config {
    noco_origin: String,
    noco_api_token: ApiToken,
}

static CONFIG: OnceLock<Config> = OnceLock::new();

pub fn init(env: &Env) -> anyhow::Result<()> {
    CONFIG
        .set(Config {
            noco_origin: env.var("NOCO_ORIGIN")?.to_string(),
            noco_api_token: ApiToken::from(env.secret("NOCO_API_TOKEN")?.to_string()),
        })
        .ok();

    Ok(())
}

fn get_config() -> &'static Config {
    CONFIG.get().expect("config not initialized")
}

pub fn noco_origin() -> String {
    get_config().noco_origin.clone()
}

pub fn noco_api_token() -> ApiToken {
    get_config().noco_api_token.clone()
}
