use worker::Url;

use crate::{
    config,
    env::{EnvId, EnvName},
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

// The URL of the client app for attendees.
pub fn app_url(env_id: &EnvId) -> anyhow::Result<Url> {
    let client_domain = config::client_domain();

    Ok(Url::parse(&format!(
        "https://{client_domain}/app/{env_id}"
    ))?)
}
