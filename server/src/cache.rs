use std::time::Duration;

use axum::{
    body::Body,
    extract::Request,
    http::{self, HeaderValue, StatusCode, Uri, header},
    middleware::Next,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use worker::{Cache, console_error};

use crate::{
    api::{DataResponseEnvelope, Freshness},
    config,
    env::{Config, EnvName},
    error::Error,
};

// How long an edge cache entry for a data endpoint is kept past its TTL. Within this window, a hit
// is still served (marked stale), but also kicks off a background refresh. See
// `put_data_cdn_cache`.
//
// This needs to comfortably cover a refresh, including a slow one against a NocoDB instance waking
// from suspend. Past it, requests fall through to the persistent cache.
const REFRESH_AHEAD_GRACE: Duration = Duration::from_secs(30);

// Set on edge cache entries for data endpoints, holding the time (as epoch milliseconds) at which
// the entry falls due for a refresh. Internal; stripped before the response is served.
const REFRESH_AT_HEADER: &str = "X-Sparklefish-Refresh-At";

// How long responses for an environment stay in the edge cache. Configurable per-environment,
// falling back to a global default.
//
// Note that `s-maxage` has second granularity, so a sub-second value here rounds down to zero and
// disables edge caching entirely for the environment.
pub fn cache_ttl(env_config: &Config) -> Duration {
    env_config
        .cache_ttl
        .map(Duration::from_millis)
        .unwrap_or_else(config::noco_default_cdn_cache_ttl)
}

// Convert a URL to a cache key. We drop query params; our API endpoints don't use them.
pub fn cache_key_uri(uri: &Uri) -> anyhow::Result<Uri> {
    let mut parts = uri.clone().into_parts();
    let path = parts
        .path_and_query
        .as_ref()
        .map_or("/", |path_and_query| path_and_query.path());
    parts.path_and_query = Some(path.parse()?);
    Ok(Uri::from_parts(parts)?)
}

#[derive(Debug, Clone, Copy, Default)]
pub struct EtagJson<T>(pub T);

impl<T> IntoResponse for EtagJson<DataResponseEnvelope<T>>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        let mut buf = Vec::new();

        // We canonicalize before computing an etag so that semantically equivalent JSON produces
        // the same etag, preventing cache misses due to differences in key ordering.
        //
        // The hash deliberately covers the whole envelope, `freshness` included. The browser makes
        // conditional requests on its own and answers a 304 with the body it stored, `freshness`
        // and all. If the same data got the same etag whether stale or fresh, a stale response
        // could resolve to a stored fresh body (so the client never checks back for the update
        // that's on its way), or a fresh one to a stored stale body (so it retries for nothing).
        let etag = {
            let mut hash_buf = Vec::new();
            serde_json_canonicalizer::to_writer(&self.0, &mut hash_buf).ok();
            let hash = blake3::hash(&hash_buf);
            HeaderValue::from_str(&format!("W/\"{}\"", hash.to_hex()))
                .expect("Invalid ETag header value")
        };

        let serialize_result = serde_json::to_writer(&mut buf, &self.0);

        match serialize_result {
            Ok(()) => {
                let mut response = (
                    [
                        (
                            header::CONTENT_TYPE,
                            HeaderValue::from_static("application/json"),
                        ),
                        (
                            header::CACHE_CONTROL,
                            HeaderValue::from_static("public, no-cache"),
                        ),
                    ],
                    buf,
                )
                    .into_response();

                response.headers_mut().insert(header::ETAG, etag);

                response
            }
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
        && weak_etags_match(&request_etag, response_etag)
    {
        return StatusCode::NOT_MODIFIED.into_response();
    }

    response
}

fn weak_etags_match(a: &str, b: &str) -> bool {
    let a = a.strip_prefix("W/").unwrap_or(a);
    let b = b.strip_prefix("W/").unwrap_or(b);
    a == b
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

// An edge cache hit for a data endpoint.
pub struct DataCacheHit {
    pub response: http::Response<Body>,

    // Whether the entry has outlived its TTL. If so, the caller should start a background refresh,
    // and must re-mark the response with `with_freshness` before serving it.
    pub refresh_due: bool,
}

// Just enough of a `DataResponseEnvelope` to re-mark it, without knowing the type of the value.
#[derive(Deserialize)]
struct CachedEnvelope {
    schema_version: u32,
    value: serde_json::Value,
}

// Re-mark an edge cache entry for a data endpoint with a different freshness.
//
// Entries are always stored as fresh, but a hit past the TTL has a refresh on its way, and the
// client needs to know to check back for it. Otherwise, while an organizer is building the
// schedule with a short TTL, their changes wouldn't show up until they reloaded a second time.
//
// This means parsing and re-serializing the body, but only for the hits that land between an entry
// falling due and its replacement being written.
pub async fn with_freshness(
    response: http::Response<Body>,
    freshness: Freshness,
) -> anyhow::Result<Response> {
    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await?;
    let cached: CachedEnvelope = serde_json::from_slice(&body)?;

    // Keep the cached schema version, rather than going through `DataResponseEnvelope::new`. An
    // entry written before a deploy that bumped it still has the old shape.
    Ok(EtagJson(DataResponseEnvelope {
        freshness,
        schema_version: cached.schema_version,
        value: cached.value,
    })
    .into_response())
}

// Look up a data endpoint in the edge cache. Entries for these are written by
// `put_data_cdn_cache`.
pub async fn get_data_cdn_cache(cache: &Cache, uri: Uri) -> Result<Option<DataCacheHit>, Error> {
    let Some(mut response) = get_cdn_cache(cache, uri).await? else {
        return Ok(None);
    };

    // An entry without the header predates refresh-ahead. Leave it to expire on its own.
    let refresh_at = response
        .headers_mut()
        .remove(REFRESH_AT_HEADER)
        .and_then(|value| value.to_str().ok()?.parse::<i64>().ok());

    let refresh_due =
        refresh_at.is_some_and(|refresh_at| chrono::Utc::now().timestamp_millis() >= refresh_at);

    Ok(Some(DataCacheHit {
        response,
        refresh_due,
    }))
}

// Put a response for a data endpoint in the edge cache, set up to be refreshed ahead of expiry.
//
// If the entry simply expired at its TTL, every request in the datacenter would miss the edge cache
// until a refresh against NocoDB finished, taking the expensive path through the persistent cache
// and coming back stale. Instead, we keep the entry for `REFRESH_AHEAD_GRACE` past its TTL and
// stamp it with when the TTL runs out. A hit after that point is still served (re-marked stale),
// but starts a background refresh (see `store::refresh_ahead`), so under steady traffic the entry
// is replaced before it ever expires.
//
// The entry falls due `ttl` after the data was fetched from NocoDB (`fetched_at_ms`), not `ttl`
// after we write it. That keeps "this entry is due" in step with "the refresh coordinator's copy
// is older than the TTL", so a refresh started by a due entry always gets newer data. Otherwise
// an organizer's edit could take up to twice the TTL to show up.
pub async fn put_data_cdn_cache(
    cache: &Cache,
    env_name: EnvName,
    ttl: Duration,
    fetched_at_ms: i64,
    uri: Uri,
    mut response: worker::Response,
) {
    // A sub-second TTL disables edge caching for the environment (see `cache_ttl`). Don't let the
    // grace period switch it back on.
    let retention = if ttl.as_secs() == 0 {
        ttl
    } else {
        ttl + REFRESH_AHEAD_GRACE
    };

    let refresh_at = fetched_at_ms + ttl.as_millis() as i64;

    if let Err(err) = response
        .headers_mut()
        .set(REFRESH_AT_HEADER, &refresh_at.to_string())
    {
        console_error!("Failed to put response in cache: {}", err);
        return;
    }

    put_cdn_cache(cache, env_name, retention, uri, response).await;
}
