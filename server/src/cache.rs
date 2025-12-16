use std::time::Duration;

use axum::{
    body::Body,
    extract::Request,
    http::{self, HeaderValue, StatusCode, Uri, header},
    middleware::Next,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use worker::{Cache, console_error};

use crate::{api::DataResponseEnvelope, env::EnvName, error::Error};

#[derive(Debug, Clone, Copy, Default)]
pub struct EtagJson<T>(pub T);

impl<T> IntoResponse for EtagJson<DataResponseEnvelope<T>>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        let mut buf = Vec::new();

        // We want to cache the data itself, not the whole response body.
        let hash_serialize_result = serde_json::to_writer(&mut buf, &self.0.value);
        let hash = blake3::hash(&buf);

        buf.clear();

        let response_serialize_result = serde_json::to_writer(&mut buf, &self.0);

        match (hash_serialize_result, response_serialize_result) {
            (Ok(()), Ok(())) => (
                [
                    (
                        header::CONTENT_TYPE,
                        HeaderValue::from_static("application/json"),
                    ),
                    (
                        header::ETAG,
                        HeaderValue::from_str(&format!("\"{}\"", hash.to_hex()))
                            .expect("Invalid ETag header value"),
                    ),
                ],
                buf,
            )
                .into_response(),
            (Ok(()), Err(err)) | (Err(err), Ok(())) | (Err(err), Err(_)) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                [(header::CONTENT_TYPE, HeaderValue::from_static("text/plain"))],
                err.to_string(),
            )
                .into_response(),
        }
    }
}

pub async fn if_none_match_middleware(request: Request, next: Next) -> impl IntoResponse {
    let request_etag = request
        .headers()
        .get(header::IF_NONE_MATCH)
        .and_then(|v| v.to_str().ok())
        .map(str::to_string);

    let response = next.run(request).await;

    let response_etag = response
        .headers()
        .get(header::ETAG)
        .and_then(|v| v.to_str().ok());

    if let (Some(request_etag), Some(response_etag)) = (request_etag, response_etag)
        && request_etag == response_etag
    {
        return StatusCode::NOT_MODIFIED.into_response();
    }

    response
}

pub async fn get_cdn_cache(cache: &Cache, uri: Uri) -> Result<Option<http::Response<Body>>, Error> {
    Ok(cache
        .get(uri.to_string(), false)
        .await
        .map_err(|err| Error::Internal(err.into()))?
        .map(http::Response::from))
}

pub fn put_cdn_cache(
    ctx: &worker::Context,
    cache: Cache,
    env_name: EnvName,
    ttl: Duration,
    uri: Uri,
    response: http::Response<Body>,
) -> Result<http::Response<Body>, Error> {
    let mut worker_response = worker::Response::try_from(response)
        .map_err(|err| Error::Internal(anyhow::Error::from(err)))?;

    let mut response_to_cache = worker_response
        .cloned()
        .map_err(|err| Error::Internal(anyhow::Error::from(err)))?;

    ctx.wait_until(async move {
        let result = async move || -> anyhow::Result<()> {
            response_to_cache
                .headers_mut()
                .append("Cache-Control", &format!("s-maxage={}", ttl.as_secs()))?;

            // Tag the cache entry with the environment name so we can invalidate the cache on a
            // per-environment basis if necessary.
            response_to_cache
                .headers_mut()
                .append("Cache-Tag", &format!("env/{}", env_name))?;

            cache.put(uri.to_string(), response_to_cache).await?;

            Ok(())
        };

        if let Err(err) = result().await {
            console_error!("Failed to put response in cache: {}", err);
        }
    });

    Ok(http::Response::from(worker_response))
}
