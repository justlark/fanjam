use std::collections::{HashMap, HashSet};

use axum::http::StatusCode;
use serde::{Serialize, de::DeserializeOwned};
use wasm_bindgen::JsValue;
use worker::{Fetch, Headers, Method, Request, RequestInit, Url, console_log};

#[derive(Debug)]
pub struct RequestBuilder {
    method: Method,
    url: String,
    params: Vec<(String, String)>,
    headers: HashMap<String, String>,
    body: Option<JsValue>,
    allowed_status: HashSet<StatusCode>,
}

impl RequestBuilder {
    pub fn new(method: Method, url: &str) -> Self {
        Self {
            method,
            url: url.to_string(),
            params: Vec::new(),
            headers: HashMap::new(),
            body: None,
            allowed_status: HashSet::new(),
        }
    }

    pub fn with_param(mut self, key: &str, value: &str) -> Self {
        self.params.push((key.to_string(), value.to_string()));
        self
    }

    pub fn with_header(mut self, key: &str, value: &str) -> Self {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }

    pub fn with_body<T>(mut self, body: &T) -> anyhow::Result<Self>
    where
        T: ?Sized + Serialize,
    {
        let json = serde_json::to_string(body)?;
        self.body = Some(JsValue::from_str(&json));

        self.headers
            .insert("Content-Type".to_string(), "application/json".to_string());

        Ok(self)
    }

    pub fn allow_status(mut self, code: StatusCode) -> Self {
        self.allowed_status.insert(code);
        self
    }

    async fn send(self) -> anyhow::Result<(StatusCode, String)> {
        let url = if self.params.is_empty() {
            Url::parse(&self.url)?
        } else {
            Url::parse_with_params(&self.url, &self.params)?
        };

        console_log!("{} {}", self.method, url.as_ref());

        let req = Request::new_with_init(
            url.as_ref(),
            &RequestInit {
                method: self.method,
                headers: Headers::from_iter(self.headers),
                body: self.body,
                ..Default::default()
            },
        )?;

        let mut resp = Fetch::Request(req).send().await?;

        let body = resp.text().await?;

        let status_code = StatusCode::from_u16(resp.status_code())?;

        if resp.status_code() >= 400
            && resp.status_code() <= 599
            && !self.allowed_status.contains(&status_code)
        {
            return Err(anyhow::anyhow!(
                "status {} for {} with body:\n{}",
                resp.status_code(),
                url.to_string(),
                body,
            ));
        }

        Ok((status_code, body))
    }

    pub async fn exec(self) -> anyhow::Result<StatusCode> {
        let (code, _) = self.send().await?;
        Ok(code)
    }

    pub async fn fetch<T>(self) -> anyhow::Result<T>
    where
        T: DeserializeOwned,
    {
        let (_, body) = self.send().await?;
        Ok(serde_json::from_str::<T>(&body)?)
    }
}
