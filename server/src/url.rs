use worker::Url;

use crate::{
    config,
    env::{EnvDomain, EnvId, EnvName},
};

// The origin of the NocoDB instance.
pub fn dash_origin(env_name: &EnvName) -> anyhow::Result<Url> {
    let base_domain = config::base_domain();

    Ok(Url::parse(&format!("https://{env_name}.{base_domain}"))?)
}

// The URL of the dashboard for organizers.
pub fn dash_url(env_name: &EnvName) -> anyhow::Result<Url> {
    Ok(Url::parse(&format!(
        "{}dashboard/",
        dash_origin(env_name)?
    ))?)
}

// The default URL of the client app.
pub fn default_app_url(env_id: &EnvId) -> anyhow::Result<Url> {
    let client_domain = config::client_domain();

    Ok(Url::parse(&format!(
        "https://{client_domain}/app/{env_id}"
    ))?)
}

// The custom domain if one is configured, and the default name otherwise.
pub fn app_url(env_id: &EnvId, custom_domain: Option<&EnvDomain>) -> anyhow::Result<Url> {
    match custom_domain {
        Some(domain) => Ok(Url::parse(&format!("https://{domain}/"))?),
        None => default_app_url(env_id),
    }
}

// The URL of a local instance of the client app for testing.
pub fn local_url(env_id: &EnvId) -> anyhow::Result<Url> {
    Ok(Url::parse(&format!("http://localhost:5173/app/{env_id}"))?)
}
