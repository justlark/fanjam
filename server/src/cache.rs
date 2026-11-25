use std::time::Duration;

use axum::{
    body::Body,
    extract::Request,
    http::{self, HeaderValue, StatusCode, Uri, header},
    middleware::Next,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use worker::Cache;

use crate::{
    api::{DataResponseEnvelope, Freshness},
    config,
    env::{Config, EnvName},
    error::Error,
    log,
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

// Set on edge cache entries for data endpoints, holding the hash of the refresh coordinator
// snapshot the entry was built from, and the ID of the Worker version that built it. Together,
// they let a refresh ask the coordinator for just a fetch time when the data hasn't changed, and
// re-stamp the entry it already has. See `restamp_data_cdn_cache`. Internal; stripped before the
// response is served.
const SOURCE_HASH_HEADER: &str = "X-Sparklefish-Source-Hash";
const VERSION_HEADER: &str = "X-Sparklefish-Version";

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
    let env_name_for_tag = env_name.clone();

    let result = async move || -> anyhow::Result<()> {
        response.headers_mut().set(
            "Cache-Control",
            &format!("public, s-maxage={}", ttl.as_secs()),
        )?;

        // Tag the cache entry with the environment name so we can invalidate the cache on a
        // per-environment basis if necessary.
        response
            .headers_mut()
            .set("Cache-Tag", &format!("env/{}", env_name_for_tag))?;

        cache.put(uri.to_string(), response).await?;

        Ok(())
    };

    if let Err(err) = result().await {
        log::error(log::Message {
            id: "edge-cache-write-failed",
            env: (&env_name).into(),
            msg: &format!("Failed putting response in the edge cache: {}", err),
        });
    }
}

// An edge cache hit for a data endpoint.
pub struct DataCacheHit {
    pub response: http::Response<Body>,

    // Whether the entry has outlived its TTL. If so, the caller should start a background refresh,
    // and must re-mark the response with `with_freshness` before serving it.
    pub refresh_due: bool,

    // The hash of the coordinator snapshot this entry was built from, if the running version of
    // the Worker built it. If the coordinator still has the same data, the refresh can re-stamp
    // this entry instead of fetching the data again.
    //
    // `None` for an entry built by any other version, because the same data might not produce
    // the same response body anymore (say, a deploy added a field or bumped the schema version).
    // Re-stamping it would keep serving the old body for as long as the data didn't change.
    pub source_hash: Option<String>,
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

    let headers = response.headers_mut();
    let source_hash = headers.remove(SOURCE_HASH_HEADER);
    let version = headers.remove(VERSION_HEADER);

    let source_hash = match (config::version_id(), version, source_hash) {
        (Some(current), Some(version), Some(hash)) if version.as_bytes() == current.as_bytes() => {
            hash.to_str().ok().map(str::to_string)
        }
        _ => None,
    };

    Ok(Some(DataCacheHit {
        response,
        refresh_due,
        source_hash,
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
//
// `source_hash` is the hash of the coordinator snapshot the response was built from, if known. See
// `DataCacheHit::source_hash`.
pub async fn put_data_cdn_cache(
    cache: &Cache,
    env_name: EnvName,
    ttl: Duration,
    fetched_at_ms: i64,
    source_hash: Option<&str>,
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

    let result = (|| -> worker::Result<()> {
        let headers = response.headers_mut();

        headers.set(REFRESH_AT_HEADER, &refresh_at.to_string())?;

        // Don't leave behind a hash from an entry we're re-stamping if we can't vouch for it.
        match (source_hash, config::version_id()) {
            (Some(hash), Some(version)) => {
                headers.set(SOURCE_HASH_HEADER, hash)?;
                headers.set(VERSION_HEADER, version)?;
            }
            _ => {
                headers.delete(SOURCE_HASH_HEADER)?;
                headers.delete(VERSION_HEADER)?;
            }
        }

        Ok(())
    })();

    if let Err(err) = result {
        log::error(log::Message {
            id: "edge-cache-write-data-failed",
            env: (&env_name).into(),
            msg: &format!("Failed putting data response in the edge cache: {}", err),
        });

        return;
    }

    put_cdn_cache(cache, env_name, retention, uri, response).await;
}

// Re-stamp an edge cache entry for a data endpoint with a newer fetch time, without changing its
// body. Used when the refresh coordinator says the data is unchanged since the snapshot the entry
// was built from (`source_hash`), so that the entry stops being due without fetching and
// re-serializing data we already have.
//
// The entry is read back from the edge cache rather than passed in, so a hit doesn't have to copy
// its body on the off chance it's needed. We only re-stamp it if it still holds the snapshot we
// asked about; if another isolate has replaced it in the meantime, or it's expired, we leave it
// alone. Either way, the next request after it falls due starts another refresh.
pub async fn restamp_data_cdn_cache(
    cache: &Cache,
    env_name: EnvName,
    ttl: Duration,
    fetched_at_ms: i64,
    source_hash: &str,
    uri: Uri,
) {
    // Resolves to the copy to put back, or why we're leaving the entry alone.
    let result = async {
        let Some(mut cached) = cache.get(uri.to_string(), false).await? else {
            return Ok::<_, worker::Error>(Err("it's no longer in the edge cache"));
        };

        // Check the version too, so that during a gradual deployment we can't take over an entry
        // another version built and vouch for its body as ours.
        if cached.headers().get(SOURCE_HASH_HEADER)?.as_deref() != Some(source_hash)
            || cached.headers().get(VERSION_HEADER)?.as_deref() != config::version_id()
        {
            return Ok(Err("it's been replaced since the refresh started"));
        }

        // Copy the response so that we can change its headers. `Headers::clone` makes an
        // independent, mutable copy.
        let headers = cached.headers().clone();

        // Drop anything the cache added on the way out. In particular, an `Age` carried over
        // into the new entry could count against its `s-maxage`.
        headers.delete("Age")?;
        headers.delete("CF-Cache-Status")?;

        let body = cached.bytes().await?;

        Ok(Ok(worker::Response::from_bytes(body)?.with_headers(headers)))
    }
    .await;

    match result {
        Ok(Ok(response)) => {
            put_data_cdn_cache(
                cache,
                env_name.clone(),
                ttl,
                fetched_at_ms,
                Some(source_hash),
                uri.clone(),
                response,
            )
            .await;

            log::log(log::Message {
                id: "edge-cache-restamp",
                env: (&env_name).into(),
                msg: &format!(
                    "Re-stamped `{}` in the edge cache; the data is unchanged.",
                    uri
                ),
            });
        }
        Ok(Err(reason)) => {
            log::log(log::Message {
                id: "edge-cache-restamp-skipped",
                env: (&env_name).into(),
                msg: &format!("Not re-stamping `{}` in the edge cache: {}.", uri, reason),
            });
        }
        Err(err) => {
            log::error(log::Message {
                id: "edge-cache-restamp-failed",
                env: (&env_name).into(),
                msg: &format!("Failed to re-stamp `{}` in the edge cache: {}", uri, err),
            });
        }
    }
}
