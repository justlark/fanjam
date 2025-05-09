use std::{fmt, iter};

use rand::Rng;
use serde::Serialize;

const ENV_ID_LENGTH: usize = 8;
const ASCII_LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";

#[derive(Debug, Clone, Serialize)]
pub struct EnvId(String);

impl fmt::Display for EnvId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub fn new_env_id() -> EnvId {
    let mut rng = rand::rng();

    EnvId(
        iter::repeat_with(|| {
            let idx = rng.random_range(0..ASCII_LOWERCASE.len());
            ASCII_LOWERCASE.chars().nth(idx).unwrap()
        })
        .take(ENV_ID_LENGTH)
        .collect(),
    )
}
