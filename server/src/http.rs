use std::{
    collections::{HashMap, HashSet},
    time::Duration,
};

use axum::http::StatusCode;
use serde::{Serialize, de::DeserializeOwned};
use wasm_bindgen::JsValue;
use worker::{Delay, Fetch, Headers, Method, Request, RequestInit, Url, console_log, console_warn};

#[derive(Debug)]
struct RetryStrategy {
    if_status: HashSet<StatusCode>,
    max_retries: u32,
    starting_backoff: Duration,
}

#[derive(Debug)]
pub struct RequestBuilder {
    method: Method,
    url: String,
    params: Vec<(String, String)>,
    headers: HashMap<String, String>,
    body: Option<JsValue>,
    allowed_status: HashSet<StatusCode>,
    retry: Option<RetryStrategy>,
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
            retry: None,
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

    pub fn with_json<T>(mut self, body: &T) -> anyhow::Result<Self>
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

    // Just in case we decide we need this later.
    #[allow(dead_code)]
    pub fn with_retry(
        mut self,
        if_status: &[StatusCode],
        max_retries: u32,
        starting_backoff: Duration,
    ) -> Self {
        self.retry = Some(RetryStrategy {
            if_status: if_status.iter().copied().collect(),
            max_retries,
            starting_backoff,
        });

        self
    }

    async fn send(self) -> anyhow::Result<(StatusCode, String)> {
        let url = if self.params.is_empty() {
            Url::parse(&self.url)?
        } else {
            Url::parse_with_params(&self.url, &self.params)?
        };

        console_log!("{} {}", self.method, url.as_ref());

        let mut retries_remaining = self.retry.as_ref().map(|r| r.max_retries).unwrap_or(0);

        let mut resp = loop {
            let req = Request::new_with_init(
                url.as_ref(),
                &RequestInit {
                    method: self.method.clone(),
                    headers: Headers::from_iter(self.headers.clone()),
                    body: self.body.clone(),
                    ..Default::default()
                },
            )?;

            let resp = Fetch::Request(req).send().await?;
            let status_code = StatusCode::from_u16(resp.status_code())?;
            let is_failed = resp.status_code() >= 400 && resp.status_code() <= 599;

            if !is_failed {
                break resp;
            }

            let retry = match &self.retry {
                Some(retry) => retry,
                None => {
                    break resp;
                }
            };

            let retry_allowed = retry.if_status.contains(&status_code);

            if !retry_allowed || retries_remaining == 0 {
                break resp;
            }

            let retry_no = retry.max_retries - retries_remaining;
            let backoff = retry.starting_backoff * (2u32.pow(retry_no));

            console_warn!(
                "Request ({}) failed with {}. Waiting {}ms. Retrying... ({} retries remaining)",
                url.to_string(),
                status_code,
                backoff.as_millis(),
                retries_remaining
            );

            Delay::from(backoff).await;

            retries_remaining -= 1;
        };

        let body = resp.text().await?;
        let status_code = StatusCode::from_u16(resp.status_code())?;
        let is_failed = resp.status_code() >= 400 && resp.status_code() <= 599;

        if is_failed && !self.allowed_status.contains(&status_code) {
            return Err(anyhow::anyhow!(
                "status {} for {} with body:\n{}",
                status_code,
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
