use std::fmt;

use crate::noco::Client;

use super::BaseId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Version(u32);

impl Version {
    pub const INITIAL: Version = Version(0);

    pub const fn next(self) -> Self {
        Self(self.0 + 1)
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
