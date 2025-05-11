use std::sync::OnceLock;

use worker::Env;

use crate::auth::ApiToken;

#[derive(Debug)]
struct Config {
    base_domain: String,
    client_domain: String,
    admin_api_token: ApiToken,
}

static CONFIG: OnceLock<Config> = OnceLock::new();

pub fn init(env: &Env) -> anyhow::Result<()> {
    CONFIG
        .set(Config {
            base_domain: env.var("BASE_DOMAIN")?.to_string(),
            client_domain: env.var("CLIENT_DOMAIN")?.to_string(),
            admin_api_token: ApiToken::try_from(
                env.secret("ADMIN_API_TOKEN")?.to_string().as_str(),
            )?,
        })
        .ok();

    Ok(())
}

fn get_config() -> &'static Config {
    CONFIG.get().expect("config not initialized")
}

pub fn base_domain() -> &'static str {
    get_config().base_domain.as_str()
}

pub fn client_domain() -> &'static str {
    get_config().client_domain.as_str()
}

pub fn admin_api_token() -> ApiToken {
    get_config().admin_api_token.clone()
}
