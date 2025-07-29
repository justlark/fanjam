use axum::{
    extract::Request,
    http::{HeaderValue, StatusCode, header},
    middleware::Next,
    response::{IntoResponse, Response},
};
use serde::Serialize;

use crate::api::DataResponseEnvelope;

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

    if let (Some(request_etag), Some(response_etag)) = (request_etag, response_etag) {
        if request_etag == response_etag {
            return StatusCode::NOT_MODIFIED.into_response();
        }
    }

    response
}
