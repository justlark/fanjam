use std::fmt::{self, Display};

use reqwest::Url;
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};

pub const DATE_FORMAT: &str = "YYYY-MM-DD";
pub const TIME_FORMAT: &str = "HH:mm";
pub const IS_TIME_12HR: bool = true;

#[derive(Debug, Clone, Copy)]
pub enum NocoViewType {
    Calendar,
}

impl NocoViewType {
    pub fn code(&self) -> u32 {
        match self {
            NocoViewType::Calendar => 6,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct BaseId(String);

impl Display for BaseId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TableId(String);

impl Display for TableId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct FieldId(String);

impl Display for FieldId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ViewId(String);

impl Display for ViewId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone)]
pub struct ApiToken(SecretString);

impl From<String> for ApiToken {
    fn from(api_token: String) -> Self {
        Self(SecretString::from(api_token))
    }
}

impl ExposeSecret<str> for ApiToken {
    fn expose_secret(&self) -> &str {
        self.0.expose_secret()
    }
}

pub type RefSetter<'a, T> = Box<dyn FnOnce(T) + 'a>;

pub fn set_ref<T>(value_ref: &mut Option<T>) -> RefSetter<T> {
    Box::new(move |id| {
        *value_ref = Some(id);
    })
}

pub fn set_nop<T>() -> RefSetter<'static, T> {
    Box::new(|_| {})
}

pub async fn check_status(resp: reqwest::Response) -> anyhow::Result<reqwest::Response> {
    #[derive(Debug, Deserialize)]
    struct ErrorResponse {
        msg: String,
        errors: Option<serde_json::Value>,
    }

    let status = resp.status();
    let url = resp.url().to_string();

    if status.is_client_error() || status.is_server_error() {
        let resp = resp.json::<ErrorResponse>().await?;

        return Err(anyhow::anyhow!(
            "Error: {} for ({}) with message ({})\n{}",
            status,
            url,
            resp.msg,
            resp.errors.unwrap_or_default(),
        ));
    }

    Ok(resp)
}

#[derive(Debug)]
pub struct Client {
    client: reqwest::Client,
    dash_origin: Url,
    api_token: ApiToken,
}

impl Client {
    pub fn new(dash_origin: Url, api_token: ApiToken) -> Self {
        Self {
            client: reqwest::Client::new(),
            dash_origin,
            api_token,
        }
    }

    // We're building this on top of the new v3 API, but we still need to fall back to the v2 API
    // for some operations that are not yet supported in v3.

    pub fn build_request_v2(&self, method: reqwest::Method, path: &str) -> reqwest::RequestBuilder {
        self.client
            .request(method, format!("{}api/v2{}", self.dash_origin, path))
            .header("Xc-Token", self.api_token.0.expose_secret())
    }

    pub fn build_request_v3(&self, method: reqwest::Method, path: &str) -> reqwest::RequestBuilder {
        self.client
            .request(method, format!("{}api/v3{}", self.dash_origin, path))
            .header("Xc-Token", self.api_token.0.expose_secret())
    }
}

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
