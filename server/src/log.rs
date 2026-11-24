use serde::Serialize;
use worker::web_sys;

use crate::env::{EnvId, EnvName};

#[derive(Debug, Clone)]
pub enum EnvRef<'a> {
    Name(&'a EnvName),
    Id(&'a EnvId),
    Global,
}

impl Serialize for EnvRef<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            EnvRef::Name(name) => serializer.serialize_str(&format!("env/{}", name)),
            EnvRef::Id(id) => serializer.serialize_str(&format!("app/{}", id)),
            EnvRef::Global => serializer.serialize_str("global"),
        }
    }
}

impl<'a> From<&'a EnvName> for EnvRef<'a> {
    fn from(name: &'a EnvName) -> Self {
        EnvRef::Name(name)
    }
}

impl<'a> From<&'a EnvId> for EnvRef<'a> {
    fn from(id: &'a EnvId) -> Self {
        EnvRef::Id(id)
    }
}

#[derive(Debug, Clone, Serialize)]
pub enum Env {
    Name(EnvName),
    Id(EnvId),
    #[allow(dead_code)]
    Global,
}

impl From<EnvName> for Env {
    fn from(name: EnvName) -> Self {
        Env::Name(name)
    }
}

impl From<EnvId> for Env {
    fn from(id: EnvId) -> Self {
        Env::Id(id)
    }
}

impl<'a> From<&'a Env> for EnvRef<'a> {
    fn from(env: &'a Env) -> Self {
        match env {
            Env::Name(name) => EnvRef::Name(name),
            Env::Id(id) => EnvRef::Id(id),
            Env::Global => EnvRef::Global,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Message<'a> {
    pub id: &'static str,
    pub env: EnvRef<'a>,
    pub msg: &'a str,
}

pub fn log<T: Serialize>(value: T) {
    let ser = serde_wasm_bindgen::Serializer::json_compatible();
    if let Ok(js) = value.serialize(&ser) {
        web_sys::console::log_1(&js);
    }
}

pub fn warn<T: Serialize>(value: T) {
    let ser = serde_wasm_bindgen::Serializer::json_compatible();
    if let Ok(js) = value.serialize(&ser) {
        web_sys::console::warn_1(&js);
    }
}

pub fn error<T: Serialize>(value: T) {
    let ser = serde_wasm_bindgen::Serializer::json_compatible();
    if let Ok(js) = value.serialize(&ser) {
        web_sys::console::error_1(&js);
    }
}
