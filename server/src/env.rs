use std::{fmt, iter};

use rand::Rng;
use serde::{Deserialize, Serialize};

// A random ID that forms part of the app URL gives to attendees.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnvId(String);

impl EnvId {
    const LEN: usize = 6;
    const POOL: &str = "0123456789";

    pub fn new() -> Self {
        let mut rng = rand::rng();

        Self(
            iter::repeat_with(|| {
                let idx = rng.random_range(0..Self::POOL.len());
                Self::POOL.chars().nth(idx).unwrap()
            })
            .take(Self::LEN)
            .collect(),
        )
    }
}

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
