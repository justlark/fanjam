use std::sync::OnceLock;

use worker::Env;

use crate::auth;
use crate::neon;

#[derive(Debug)]
struct Config {
    base_domain: String,
    client_domain: String,
    admin_api_token: auth::ApiToken,
    neon_api_token: neon::ApiToken,
    neon_org_id: String,
    neon_default_branch_name: String,
}

static CONFIG: OnceLock<Config> = OnceLock::new();

pub fn init(env: &Env) -> anyhow::Result<()> {
    CONFIG
        .set(Config {
            base_domain: env.var("BASE_DOMAIN")?.to_string(),
            client_domain: env.var("CLIENT_DOMAIN")?.to_string(),
            admin_api_token: auth::ApiToken::try_from(
                env.secret("ADMIN_API_TOKEN")?.to_string().as_str(),
            )?,
            neon_api_token: neon::ApiToken::from(env.secret("NEON_API_TOKEN")?.to_string()),
            neon_org_id: env.secret("NEON_ORG_ID")?.to_string(),
            neon_default_branch_name: env.secret("NEON_DEFAULT_BRANCH_NAME")?.to_string(),
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

pub fn admin_api_token() -> auth::ApiToken {
    get_config().admin_api_token.clone()
}

pub fn neon_api_token() -> neon::ApiToken {
    get_config().neon_api_token.clone()
}

pub fn neon_org_id() -> String {
    get_config().neon_org_id.clone()
}

pub fn neon_default_branch_name() -> neon::BranchName {
    get_config().neon_default_branch_name.clone().into()
}
