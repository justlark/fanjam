//! VAPID (Voluntary Application Server Identification) JWT signing per
//! [RFC 8292]. We use the `vapid` authentication scheme — the variant where
//! the JWT and the public key go in a single `Authorization` header.
//!
//! [RFC 8292]: https://datatracker.ietf.org/doc/html/rfc8292

use base64::prelude::*;
use p256::{
    SecretKey,
    ecdsa::{Signature, SigningKey, signature::Signer},
    elliptic_curve::sec1::ToEncodedPoint,
};
use serde::Serialize;
use worker::Url;

/// How long a VAPID JWT is valid for. RFC 8292 §2 caps this at 24 hours and
/// recommends shorter; 12h is a comfortable balance between cache reuse and
/// blast radius if the signed token leaks. The JWT is per-audience (per
/// push-service origin) so a leaked token can only target that one host.
const TOKEN_TTL_SECS: i64 = 12 * 60 * 60;

/// Long-lived VAPID identity for our application server. Built once at
/// startup from the worker's `VAPID_PRIVATE_KEY` secret and reused across
/// every push. The wrapped `SigningKey` is the actual signer; we cache the
/// pre-encoded public key alongside it because every JWT we issue carries
/// the same `k=` value.
#[derive(Clone)]
pub struct VapidKey {
    signing_key: SigningKey,
    public_key_b64: String,
    subject: String,
}

impl VapidKey {
    /// Parse a base64url-encoded 32-byte P-256 private scalar (the format
    /// produced by the `openssl` recipe in the deploy docs) into a usable
    /// key, paired with the contact subject string (a `mailto:` or `https:`
    /// URL identifying us to the push service operator).
    pub fn from_base64url(private_b64url: &str, subject: impl Into<String>) -> anyhow::Result<Self> {
        // Tolerate the occasional stray `=` if the secret was pasted with
        // padding from an external tool.
        let trimmed = private_b64url.trim().trim_end_matches('=');
        let private_bytes = BASE64_URL_SAFE_NO_PAD
            .decode(trimmed)
            .map_err(|e| anyhow::anyhow!("VAPID private key is not valid base64url: {e}"))?;
        let secret = SecretKey::from_slice(&private_bytes)
            .map_err(|e| anyhow::anyhow!("VAPID private key is not a valid P-256 scalar: {e}"))?;
        let signing_key = SigningKey::from(&secret);
        let public_point = secret.public_key().to_encoded_point(false);
        let public_key_b64 = BASE64_URL_SAFE_NO_PAD.encode(public_point.as_bytes());
        Ok(Self {
            signing_key,
            public_key_b64,
            subject: subject.into(),
        })
    }

    /// SEC1-uncompressed VAPID public key, base64url-encoded — what we send
    /// to clients via `VITE_VAPID_PUBLIC_KEY` and what we put in the `k=`
    /// parameter of every `Authorization` header.
    pub fn public_key_b64(&self) -> &str {
        &self.public_key_b64
    }
}

/// Serializable JWT claim set per RFC 8292 §2.
#[derive(Serialize)]
struct Claims<'a> {
    aud: &'a str,
    exp: i64,
    sub: &'a str,
}

/// Build the full `Authorization: vapid t=…, k=…` header value targeted at
/// `audience` (the origin of the push subscription endpoint). `issued_at`
/// is a Unix timestamp in seconds; the actual `exp` claim is
/// `issued_at + TOKEN_TTL_SECS`.
pub fn build_authorization_header(
    key: &VapidKey,
    audience: &str,
    issued_at: i64,
) -> anyhow::Result<String> {
    // The JWT header is a constant — VAPID only allows ES256. Hard-coding
    // the bytes saves a `serde_json::to_string` and keeps the base64
    // encoding stable (no risk of key-order shuffling).
    const HEADER_B64: &str = "eyJ0eXAiOiJKV1QiLCJhbGciOiJFUzI1NiJ9";
    debug_assert_eq!(
        HEADER_B64,
        BASE64_URL_SAFE_NO_PAD.encode(br#"{"typ":"JWT","alg":"ES256"}"#),
    );

    let claims = Claims {
        aud: audience,
        exp: issued_at + TOKEN_TTL_SECS,
        sub: &key.subject,
    };
    let claims_json = serde_json::to_string(&claims)?;
    let claims_b64 = BASE64_URL_SAFE_NO_PAD.encode(claims_json.as_bytes());

    let signing_input = format!("{HEADER_B64}.{claims_b64}");
    // p256's `SigningKey::sign` uses RFC 6979 deterministic ECDSA, so the
    // same input always produces the same JWT — convenient for tests and
    // avoids needing a CSPRNG in the signing path.
    let sig: Signature = key.signing_key.sign(signing_input.as_bytes());
    let sig_b64 = BASE64_URL_SAFE_NO_PAD.encode(sig.to_bytes());

    Ok(format!(
        "vapid t={signing_input}.{sig_b64}, k={}",
        key.public_key_b64
    ))
}

/// Derive the VAPID `aud` claim — the origin of the push subscription
/// endpoint URL, e.g. `https://fcm.googleapis.com`. Push services reject
/// JWTs with a mismatched audience to prevent token replay across services.
pub fn audience_from_endpoint(endpoint: &str) -> anyhow::Result<String> {
    let url = Url::parse(endpoint)
        .map_err(|e| anyhow::anyhow!("subscription endpoint is not a valid URL: {e}"))?;
    Ok(url.origin().ascii_serialization())
}

#[cfg(test)]
mod tests {
    use super::*;
    use p256::ecdsa::{VerifyingKey, signature::Verifier};

    /// Test private key from RFC 8291 Appendix A.2; conveniently it's also
    /// a valid VAPID private key (any 32-byte P-256 scalar is). Using a
    /// fixed key lets the test assert deterministic outputs.
    const TEST_PRIVATE_B64: &str = "yfWPiYE-n46HLnH0KqZOF1fJJU3MYrct3AELtAQ-oRw";

    fn parts(header: &str) -> (Vec<&str>, &str) {
        // "vapid t=<jwt>, k=<pub>"
        let payload = header.strip_prefix("vapid ").expect("scheme prefix");
        let (t_part, k_part) = payload.split_once(", ").expect("t,k delimiter");
        let jwt = t_part.strip_prefix("t=").expect("t= prefix");
        let pk = k_part.strip_prefix("k=").expect("k= prefix");
        (jwt.split('.').collect(), pk)
    }

    #[test]
    fn audience_strips_path() {
        assert_eq!(
            audience_from_endpoint(
                "https://updates.push.services.mozilla.com/wpush/v2/abcdef"
            )
            .unwrap(),
            "https://updates.push.services.mozilla.com",
        );
    }

    #[test]
    fn header_has_three_jwt_parts_and_correct_public_key() {
        let key = VapidKey::from_base64url(TEST_PRIVATE_B64, "mailto:test@example.com").unwrap();
        let header =
            build_authorization_header(&key, "https://example.push", 1_700_000_000).unwrap();
        let (jwt_parts, pk) = parts(&header);
        assert_eq!(jwt_parts.len(), 3);
        // The `k=` value is the same as the VapidKey's public key.
        assert_eq!(pk, key.public_key_b64());
    }

    #[test]
    fn header_payload_encodes_expected_claims() {
        let key = VapidKey::from_base64url(TEST_PRIVATE_B64, "mailto:test@example.com").unwrap();
        let issued_at = 1_700_000_000_i64;
        let header =
            build_authorization_header(&key, "https://example.push", issued_at).unwrap();
        let (jwt_parts, _) = parts(&header);
        let payload = BASE64_URL_SAFE_NO_PAD.decode(jwt_parts[1]).unwrap();
        let payload: serde_json::Value = serde_json::from_slice(&payload).unwrap();
        assert_eq!(payload["aud"], "https://example.push");
        assert_eq!(payload["sub"], "mailto:test@example.com");
        // `exp` is issued_at + 12h.
        assert_eq!(payload["exp"], issued_at + 12 * 60 * 60);
    }

    #[test]
    fn header_signature_verifies_against_public_key() {
        let key = VapidKey::from_base64url(TEST_PRIVATE_B64, "mailto:test@example.com").unwrap();
        let header =
            build_authorization_header(&key, "https://example.push", 1_700_000_000).unwrap();
        let (jwt_parts, _) = parts(&header);
        let signing_input = format!("{}.{}", jwt_parts[0], jwt_parts[1]);
        let sig_bytes = BASE64_URL_SAFE_NO_PAD.decode(jwt_parts[2]).unwrap();
        let signature = Signature::from_slice(&sig_bytes).expect("valid sig encoding");

        let verifying_key: &VerifyingKey = key.signing_key.verifying_key();
        verifying_key
            .verify(signing_input.as_bytes(), &signature)
            .expect("signature must verify under VAPID public key");
    }

    #[test]
    fn header_is_deterministic_for_fixed_inputs() {
        // RFC 6979 deterministic signing means two builds with the same
        // inputs produce byte-identical JWTs — useful both as a sanity check
        // and as a regression guard if we ever switch signing backends.
        let key = VapidKey::from_base64url(TEST_PRIVATE_B64, "mailto:test@example.com").unwrap();
        let a = build_authorization_header(&key, "https://example.push", 42).unwrap();
        let b = build_authorization_header(&key, "https://example.push", 42).unwrap();
        assert_eq!(a, b);
    }
}
