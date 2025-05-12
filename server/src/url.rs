use reqwest::Url;

use crate::{
    config,
    env::{EnvId, EnvName},
};

pub fn dash_origin(env_name: &EnvName) -> anyhow::Result<Url> {
    let base_domain = config::base_domain();

    Ok(Url::parse(&format!(
        "https://{}.{}",
        env_name, base_domain
    ))?)
}

pub fn dash_url(dash_origin: Url) -> anyhow::Result<Url> {
    Ok(Url::parse(&format!("{}dashboard/", dash_origin))?)
}

pub fn app_url(env_id: &EnvId) -> anyhow::Result<Url> {
    let client_domain = config::client_domain();

    Ok(Url::parse(&format!(
        "https://{}/app/{}",
        client_domain, env_id
    ))?)
}
