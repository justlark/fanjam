use std::sync::OnceLock;

use worker::Env;

#[derive(Debug)]
struct Config {
    base_domain: String,
    client_domain: String,
}

static CONFIG: OnceLock<Config> = OnceLock::new();

pub fn init(env: &Env) -> anyhow::Result<()> {
    CONFIG
        .set(Config {
            base_domain: env.var("BASE_DOMAIN")?.to_string(),
            client_domain: env.var("CLIENT_DOMAIN")?.to_string(),
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
