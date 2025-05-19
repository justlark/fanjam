use std::{fmt, iter};

use rand::Rng;
use serde::{Deserialize, Serialize};

// A random ID that forms part of the app URL gives to attendees.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvId(String);

impl EnvId {
    const LEN: usize = 8;
    const POOL: &str = "abcdefghijklmnopqrstuvwxyz";

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
