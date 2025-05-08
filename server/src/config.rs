use std::sync::OnceLock;

use worker::Env;

#[derive(Debug)]
struct Config {
    base_domain: String,
}

static CONFIG: OnceLock<Config> = OnceLock::new();

pub fn init(env: &Env) -> anyhow::Result<()> {
    CONFIG
        .set(Config {
            base_domain: env.var("BASE_DOMAIN")?.to_string(),
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
