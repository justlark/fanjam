//! Push delivery client: ties together encryption + VAPID JWT + HTTP POST
//! against the subscription endpoint. One `Client` per environment is the
//! intended usage — it holds the long-lived VAPID identity so we don't
//! re-parse the private key on every notification.

use axum::http::StatusCode;
use base64::prelude::*;
use serde::Deserialize;
use worker::Method;

use crate::http;
use crate::push::encrypt::{PUBLIC_KEY_LEN, Sender};
use crate::push::vapid::{self, VapidKey};

/// Default time-to-live for a push message at the push service, in seconds.
/// 24 hours is what every reference client uses; if a subscriber is offline
/// longer than this they just miss the announcement.
const DEFAULT_TTL_SECS: u32 = 24 * 60 * 60;

/// Length of the per-subscription `auth` secret. RFC 8291 §3.2.
const AUTH_LEN: usize = 16;

/// JSON shape produced by `PushSubscription.toJSON()` in the browser; this
/// is what the client POSTs to `/apps/{env}/subscription` and what we store
/// in KV. Field names match the wire format exactly.
#[derive(Debug, Clone, Deserialize)]
pub struct Subscription {
    pub endpoint: String,
    pub keys: SubscriptionKeys,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SubscriptionKeys {
    /// SEC1-uncompressed P-256 public key, base64url-encoded (65 bytes raw).
    pub p256dh: String,
    /// Auth secret, base64url-encoded (16 bytes raw).
    pub auth: String,
}

/// Outcome of a single delivery attempt — bundled so the caller can decide
/// whether to retry, log, or evict the subscription from KV.
#[derive(Debug)]
pub enum DeliveryOutcome {
    /// Push service accepted the message (2xx).
    Delivered,
    /// Subscription is permanently invalid (404 or 410). Caller should
    /// delete it from KV so we don't keep trying.
    SubscriptionGone,
    /// Some other non-success status (e.g. 413 payload too large, 429 rate
    /// limited, 5xx). Surfaced but not retried; we trust the push service's
    /// own ttl/retry semantics for the next announcement.
    OtherStatus(StatusCode),
}

/// Reusable per-environment push sender.
pub struct Client {
    vapid: VapidKey,
}

impl Client {
    pub fn new(vapid: VapidKey) -> Self {
        Self { vapid }
    }

    /// Encrypt `payload` for `subscription` and POST it to the push service.
    /// Uses a fresh ephemeral keypair + salt per call (required by RFC 8291).
    pub async fn send(
        &self,
        subscription: &Subscription,
        payload: &[u8],
    ) -> anyhow::Result<DeliveryOutcome> {
        let p256dh =
            decode_b64url_fixed::<PUBLIC_KEY_LEN>(&subscription.keys.p256dh, "p256dh")?;
        let auth = decode_b64url_fixed::<AUTH_LEN>(&subscription.keys.auth, "auth")?;

        let sender = Sender::random();
        let encrypted = sender.encrypt(&p256dh, &auth, payload)?;

        let audience = vapid::audience_from_endpoint(&subscription.endpoint)?;
        let issued_at = chrono::Utc::now().timestamp();
        let auth_header = vapid::build_authorization_header(&self.vapid, &audience, issued_at)?;

        let status = http::RequestBuilder::new(Method::Post, &subscription.endpoint)
            .with_header("Authorization", &auth_header)
            .with_header("Content-Encoding", "aes128gcm")
            .with_header("TTL", &DEFAULT_TTL_SECS.to_string())
            .with_bytes(&encrypted.body, "application/octet-stream")
            // 404/410 mean the subscription is gone; let those bubble back so
            // we can evict instead of treating them as transport errors.
            .allow_status(StatusCode::NOT_FOUND)
            .allow_status(StatusCode::GONE)
            .exec()
            .await?;

        Ok(match status {
            s if s.is_success() => DeliveryOutcome::Delivered,
            StatusCode::NOT_FOUND | StatusCode::GONE => DeliveryOutcome::SubscriptionGone,
            other => DeliveryOutcome::OtherStatus(other),
        })
    }

    /// The VAPID public key the client also has in its `VITE_VAPID_PUBLIC_KEY`
    /// build var. Exposed for the public config endpoint so the client can
    /// double-check the key it was built with against what the server is
    /// signing with (mismatch is a deploy bug, not a runtime condition).
    #[allow(dead_code)] // wired up in slice 2
    pub fn vapid_public_key_b64(&self) -> &str {
        self.vapid.public_key_b64()
    }
}

/// Decode a base64url (with or without padding) string into a fixed-size
/// array. Used for both subscription key fields, which have known lengths.
fn decode_b64url_fixed<const N: usize>(s: &str, name: &str) -> anyhow::Result<[u8; N]> {
    let trimmed = s.trim().trim_end_matches('=');
    let bytes = BASE64_URL_SAFE_NO_PAD
        .decode(trimmed)
        .map_err(|e| anyhow::anyhow!("subscription {name} is not valid base64url: {e}"))?;
    bytes
        .try_into()
        .map_err(|v: Vec<u8>| {
            anyhow::anyhow!(
                "subscription {name} is {} bytes, expected {N}",
                v.len()
            )
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_subscription_key_with_wrong_length() {
        // 43 base64url chars decode to 32 bytes, but a p256dh must be 65.
        // We bail in validation, not at the network call.
        let err = decode_b64url_fixed::<PUBLIC_KEY_LEN>(&"A".repeat(43), "p256dh").unwrap_err();
        assert!(err.to_string().contains("expected 65"));
    }

    #[test]
    fn accepts_padded_base64url() {
        // PushSubscription serialization may include trailing `=` padding
        // depending on the browser; the decoder should accept both.
        let unpadded = "BTBZMqHH6r4Tts7J_aSIgg";
        let padded = format!("{unpadded}==");
        assert_eq!(
            decode_b64url_fixed::<16>(unpadded, "auth").unwrap(),
            decode_b64url_fixed::<16>(&padded, "auth").unwrap(),
        );
    }
}
