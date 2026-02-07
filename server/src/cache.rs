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

        // We canonicalize the JSON response so that semantically equivalent JSON produces the same
        // response byte-for-byte which prevents cache misses due to differences in key ordering.
        let serialize_result = serde_json_canonicalizer::to_writer(&self.0, &mut buf);
        let hash = blake3::hash(&buf);

        match serialize_result {
            Ok(()) => (
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
                    (
                        header::CACHE_CONTROL,
                        HeaderValue::from_static("public, no-cache"),
                    ),
                ],
                buf,
            )
                .into_response(),
            Err(err) => (
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
        .map(http::Response::from)
        .map(|mut response| {
            response.headers_mut().insert(
                header::CACHE_CONTROL,
                HeaderValue::from_static("public, no-cache"),
            );
            response
        }))
}

pub async fn put_cdn_cache(
    cache: &Cache,
    env_name: EnvName,
    ttl: Duration,
    uri: Uri,
    mut response: worker::Response,
) {
    let result = async move || -> anyhow::Result<()> {
        response.headers_mut().set(
            "Cache-Control",
            &format!("public, s-maxage={}", ttl.as_secs()),
        )?;

        // Tag the cache entry with the environment name so we can invalidate the cache on a
        // per-environment basis if necessary.
        response
            .headers_mut()
            .set("Cache-Tag", &format!("env/{}", env_name))?;

        cache.put(uri.to_string(), response).await?;

        Ok(())
    };

    if let Err(err) = result().await {
        console_error!("Failed to put response in cache: {}", err);
    }
}
