use reqwest::Url;

use crate::{config, env::EnvId, noco::BaseId};

pub fn dash_origin(dash_domain: &str) -> anyhow::Result<Url> {
    let base_domain = config::base_domain();

    Ok(Url::parse(&format!(
        "https://{}.{}",
        dash_domain, base_domain
    ))?)
}

pub fn dash_url(dash_origin: Url, base_id: BaseId) -> anyhow::Result<Url> {
    Ok(Url::parse(&format!(
        "{}dashboard/#/nc/{}",
        dash_origin, base_id
    ))?)
}

pub fn app_url(env_id: EnvId) -> anyhow::Result<Url> {
    let client_domain = config::client_domain();

    Ok(Url::parse(&format!(
        "https://{}/app/?id={}",
        client_domain, env_id
    ))?)
}
