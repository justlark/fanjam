use std::fmt;

use serde::{Deserialize, Serialize};
use worker::Url;

// A random ID that forms part of the app URL gives to attendees.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnvId(String);

impl From<String> for EnvId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl fmt::Display for EnvId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// The internal name for the environment, used to identify resources in the infrastructure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvName(String);

impl From<String> for EnvName {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl fmt::Display for EnvName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// A custom hostname ("vanity domain") that an instance of the client can be served from.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct EnvDomain(String);

impl EnvDomain {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for EnvDomain {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            anyhow::bail!("Custom domain must not be empty.");
        }

        let parsed = Url::parse(&format!("https://{trimmed}/"))
            .map_err(|err| anyhow::anyhow!("Custom domain is not a valid hostname: {err}"))?;

        let host = parsed
            .host_str()
            .ok_or_else(|| anyhow::anyhow!("Custom domain is missing a hostname."))?;

        if parsed.port().is_some() {
            anyhow::bail!("Custom domain must not include a port.");
        }
        if parsed.path() != "/" {
            anyhow::bail!("Custom domain must not include a path.");
        }
        if parsed.query().is_some() || parsed.fragment().is_some() {
            anyhow::bail!("Custom domain must not include a query string or fragment.");
        }
        if !parsed.username().is_empty() || parsed.password().is_some() {
            anyhow::bail!("Custom domain must not include userinfo.");
        }

        // Require at least one dot so we don't accept single-label hostnames like `localhost`.
        if !host.contains('.') {
            anyhow::bail!("Custom domain must contain at least one dot.");
        }

        Ok(Self(host.to_ascii_lowercase()))
    }
}

impl fmt::Display for EnvDomain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// Environment-specific configuration. This should be a flat key-value map.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Config {
    pub timezone: Option<String>,
    pub cache_ttl: Option<u64>,
    pub hide_announcements: Option<bool>,
    pub use_feedback: Option<bool>,
    pub feedback_icon: Option<String>,
    pub feedback_title: Option<String>,
    pub feedback_detail: Option<String>,
    pub feedback_url: Option<String>,
    pub use_schedule_sharing: Option<bool>,
    pub config_db_host: Option<String>,
    pub config_db_port: Option<u16>,
    pub config_db_name: Option<String>,
    pub config_db_user: Option<String>,
    pub config_db_password: Option<String>,
    pub use_custom_icon: Option<bool>,
    pub favicon_name: Option<String>,
    pub opengraph_icon_name: Option<String>,
    pub opengraph_icon_type: Option<String>,
    pub opengraph_icon_alt: Option<String>,
    pub pwa_short_app_name: Option<String>,
    pub pwa_background_color: Option<String>,
    pub pwa_icon_any_name: Option<String>,
    pub pwa_icon_any_type: Option<String>,
    pub pwa_icon_any_sizes: Option<String>,
    pub pwa_icon_maskable_name: Option<String>,
    pub pwa_icon_maskable_type: Option<String>,
    pub pwa_icon_maskable_sizes: Option<String>,
}

// Documentation and metadata for each config key in the environment-specific configuration. Keep
// this up to date.
pub const CONFIG_SPEC: &str = include_str!("./config-spec.json");
