use std::{
    collections::{HashMap, HashSet},
    fmt,
    time::Duration,
};

use axum::http::{HeaderMap, HeaderValue, StatusCode};
use serde::{Serialize, de::DeserializeOwned};
use wasm_bindgen::JsValue;
use worker::{
    Delay, Fetch, Headers, HttpMetadata, Method, Object, Request, RequestInit, ResponseBody, Url,
    console_log, console_warn,
};

use crate::error::Error;

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
    status_map: HashMap<StatusCode, StatusCode>,
    retry: Option<RetryStrategy>,
}

#[derive(Debug)]
pub struct StatusError {
    code: StatusCode,
    url: Url,
    body: String,
}

impl StatusError {
    pub fn new(code: StatusCode, url: Url, body: String) -> Self {
        Self { code, url, body }
    }
}

impl fmt::Display for StatusError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "status {} for {} with body:\n{}",
            self.code, self.url, self.body,
        )
    }
}

impl std::error::Error for StatusError {}

impl RequestBuilder {
    pub fn new(method: Method, url: &str) -> Self {
        Self {
            method,
            url: url.to_string(),
            params: Vec::new(),
            headers: HashMap::new(),
            body: None,
            allowed_status: HashSet::new(),
            status_map: HashMap::new(),
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

    // TODO: Remove this if we find we don't end up needing it later.
    //
    // This applies before `allow_status` and `with_retry`.
    #[allow(dead_code)]
    pub fn map_status(mut self, from: StatusCode, to: StatusCode) -> Self {
        if from != to {
            self.status_map.insert(from, to);
        }

        self
    }

    // TODO: Remove this if we find we don't end up needing it later.
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

        let (mut resp, status_code) = loop {
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
            let original_status = StatusCode::from_u16(resp.status_code())?;
            let status_code = self
                .status_map
                .get(&original_status)
                .cloned()
                .unwrap_or(original_status);
            let is_failed = status_code.as_u16() >= 400 && status_code.as_u16() <= 599;

            if !is_failed {
                break (resp, status_code);
            }

            let retry = match &self.retry {
                Some(retry) => retry,
                None => {
                    break (resp, status_code);
                }
            };

            let retry_allowed = retry.if_status.contains(&status_code);

            if !retry_allowed || retries_remaining == 0 {
                break (resp, status_code);
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
        let is_failed = status_code.as_u16() >= 400 && status_code.as_u16() <= 599;

        if is_failed && !self.allowed_status.contains(&status_code) {
            return Err(StatusError::new(status_code, url, body.clone()).into());
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

// The R2 bindings have an API which should do this for us, but someone at Cloudflare seems to be
// confused about how move semantics work in Rust:
//
// https://github.com/cloudflare/workers-rs/issues/302
pub fn write_http_metadata(
    metadata: &HttpMetadata,
    headers: &mut HeaderMap<HeaderValue>,
) -> anyhow::Result<()> {
    if let Some(content_type) = &metadata.content_type {
        headers.insert("Content-Type", content_type.parse()?);
    }

    if let Some(content_language) = &metadata.content_language {
        headers.insert("Content-Language", content_language.parse()?);
    }

    if let Some(content_disposition) = &metadata.content_disposition {
        headers.insert("Content-Disposition", content_disposition.parse()?);
    }

    if let Some(content_encoding) = &metadata.content_encoding {
        headers.insert("Content-Encoding", content_encoding.parse()?);
    }

    if let Some(cache_control) = &metadata.cache_control {
        headers.insert("Cache-Control", cache_control.parse()?);
    }

    if let Some(cache_expiry) = &metadata.cache_expiry {
        headers.insert("Expires", cache_expiry.to_string().parse()?);
    }

    Ok(())
}

pub fn body_from_response_body(body: ResponseBody) -> anyhow::Result<worker::Body> {
    Ok(match body {
        worker::ResponseBody::Empty => worker::Body::empty(),
        // Is there a more elegant way to make this conversion?
        worker::ResponseBody::Body(bytes) => {
            worker::Body::from_stream(futures::stream::once(async {
                Result::<_, Error>::Ok(bytes)
            }))
            .map_err(|err| Error::Internal(err.into()))?
        }
        worker::ResponseBody::Stream(readable_stream) => worker::Body::new(readable_stream),
    })
}

pub fn http_headers_from_object(object: &Object) -> anyhow::Result<HeaderMap<HeaderValue>> {
    let mut response_headers = HeaderMap::new();
    let http_metadata = object.http_metadata();
    let http_etag = object.http_etag();

    write_http_metadata(&http_metadata, &mut response_headers).map_err(Error::Internal)?;

    response_headers.insert(
        "ETag",
        http_etag
            .parse()
            .map_err(|err| Error::Internal(anyhow::Error::from(err)))?,
    );

    Ok(response_headers)
}
