use std::{fmt, iter};

use rand::Rng;
use serde::{Deserialize, Serialize};

const ENV_ID_LENGTH: usize = 8;
const ASCII_LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvId(String);

impl EnvId {
    pub fn new() -> Self {
        let mut rng = rand::rng();

        Self(
            iter::repeat_with(|| {
                let idx = rng.random_range(0..ASCII_LOWERCASE.len());
                ASCII_LOWERCASE.chars().nth(idx).unwrap()
            })
            .take(ENV_ID_LENGTH)
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
