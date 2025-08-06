use std::{fmt, str::FromStr};

use serde::{Deserialize, Serialize};

use crate::noco::Client;

use super::BaseId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Version(u32);

impl Version {
    pub const INITIAL: Version = Version(0);

    pub const fn next(self) -> Self {
        Self(self.0 + 1)
    }

    pub const fn prev(self) -> Self {
        Self(self.0 - 1)
    }
}

impl FromStr for Version {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<u32>()
            .map(Version)
            .map_err(|_| anyhow::anyhow!("Invalid migration version number string: {}", s))
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub trait Migration<'a> {
    const INDEX: Version;

    fn new(client: &'a Client) -> Self;

    async fn migrate(&self, base_id: BaseId) -> anyhow::Result<()>;
}
