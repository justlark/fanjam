//! Web Push message encryption per [RFC 8291] (the `aes128gcm`
//! content-encoding from [RFC 8188]).
//!
//! [RFC 8291]: https://datatracker.ietf.org/doc/html/rfc8291
//! [RFC 8188]: https://datatracker.ietf.org/doc/html/rfc8188

use aes_gcm::{Aes128Gcm, Key, KeyInit, Nonce, aead::Aead};
use hkdf::Hkdf;
use p256::{
    PublicKey, SecretKey,
    elliptic_curve::{ecdh::diffie_hellman, sec1::ToEncodedPoint},
};
use rand_core::{OsRng, RngCore};
use sha2::Sha256;

/// Record size advertised in the RFC 8188 framing header. The application
/// server picks this; 4096 is the conventional value used by every Web Push
/// implementation, and our payloads are well under one record.
const RECORD_SIZE: u32 = 4096;

/// Length of the random salt used in the RFC 8188 framing header / second
/// HKDF stage. RFC 8188 §2.1.
const SALT_LEN: usize = 16;

/// Length of the per-subscription `auth` secret carried by the user agent in
/// the `PushSubscription`. RFC 8291 §3.2.
const AUTH_LEN: usize = 16;

/// Length of an uncompressed SEC1 P-256 public key (`0x04 || X || Y`).
pub const PUBLIC_KEY_LEN: usize = 65;

/// An encrypted Web Push payload, ready to be POSTed to the subscription
/// endpoint as the request body.
pub struct Encrypted {
    /// The full encrypted body: RFC 8188 framing header (salt, rs, idlen,
    /// keyid) followed by the AES-128-GCM ciphertext.
    pub body: Vec<u8>,

    /// SEC1-uncompressed public key of the ephemeral keypair used to derive
    /// the shared secret for this message. Same bytes that are embedded in
    /// `body` as the `keyid`; we expose it separately because the VAPID
    /// `Authorization` header carries the *VAPID* (long-lived) public key in
    /// its `k=` parameter, which is unrelated to this ephemeral one.
    pub server_public_key: [u8; PUBLIC_KEY_LEN],
}

/// A one-shot encryption context: a fresh ephemeral P-256 keypair plus a
/// fresh 16-byte salt, both used exactly once per outgoing message.
///
/// Construct one of these via [`Sender::random`] for each push you send.
pub struct Sender {
    server_secret: SecretKey,
    salt: [u8; SALT_LEN],
}

impl Sender {
    /// Generate a fresh ephemeral keypair and salt for a single message.
    pub fn random() -> Self {
        let mut salt = [0u8; SALT_LEN];
        OsRng.fill_bytes(&mut salt);
        Self {
            server_secret: SecretKey::random(&mut OsRng),
            salt,
        }
    }

    /// Construct a `Sender` from raw 32-byte private scalar and 16-byte salt.
    /// Used by tests to drive the encryption with the deterministic inputs
    /// from RFC 8291 Appendix A.
    #[cfg(test)]
    pub fn from_raw(server_secret: [u8; 32], salt: [u8; SALT_LEN]) -> anyhow::Result<Self> {
        let server_secret = SecretKey::from_slice(&server_secret)
            .map_err(|e| anyhow::anyhow!("invalid server secret key: {e}"))?;
        Ok(Self { server_secret, salt })
    }

    /// Encrypt a single message for the given subscription's user-agent
    /// public key + auth secret. See RFC 8291 §3.4.
    pub fn encrypt(
        &self,
        ua_public: &[u8; PUBLIC_KEY_LEN],
        auth: &[u8; AUTH_LEN],
        message: &[u8],
    ) -> anyhow::Result<Encrypted> {
        // SEC1-uncompressed encoding of the sender's ephemeral public key.
        // This is the value used both as the HKDF info argument and as the
        // `keyid` in the framing header that the receiver uses to recompute
        // the shared secret.
        let server_public_point = self.server_secret.public_key().to_encoded_point(false);
        let server_public_slice = server_public_point.as_bytes();
        // P-256 always serializes to exactly 65 bytes here; assert that
        // invariant instead of paying for runtime checks elsewhere.
        debug_assert_eq!(server_public_slice.len(), PUBLIC_KEY_LEN);
        let mut server_public = [0u8; PUBLIC_KEY_LEN];
        server_public.copy_from_slice(server_public_slice);

        // ECDH(server_secret, ua_public) — the raw shared secret. RFC 8291
        // calls this `ecdh_secret`.
        let ua_pk = PublicKey::from_sec1_bytes(ua_public)
            .map_err(|e| anyhow::anyhow!("invalid user-agent public key: {e}"))?;
        let shared = diffie_hellman(self.server_secret.to_nonzero_scalar(), ua_pk.as_affine());
        let shared_bytes = shared.raw_secret_bytes();

        // First HKDF stage (RFC 8291 §3.4):
        //
        //   IKM = HKDF-Expand(
        //             HKDF-Extract(salt = auth_secret, IKM = ecdh_secret),
        //             info = "WebPush: info" || 0x00 || ua_public || as_public,
        //             L   = 32)
        //
        // Binding the IKM to both public keys means a leaked `auth` secret
        // for one subscription can't be reused to derive keys for another.
        let stage1 = Hkdf::<Sha256>::new(Some(auth.as_slice()), shared_bytes.as_slice());
        let mut key_info = Vec::with_capacity(14 + 2 * PUBLIC_KEY_LEN);
        key_info.extend_from_slice(b"WebPush: info\0");
        key_info.extend_from_slice(ua_public);
        key_info.extend_from_slice(&server_public);
        let mut ikm = [0u8; 32];
        stage1
            .expand(&key_info, &mut ikm)
            .map_err(|e| anyhow::anyhow!("HKDF expand for IKM failed: {e}"))?;

        // Second HKDF stage — RFC 8188 §2.2 / §2.3 content-encryption key
        // derivation, with the random salt from the framing header.
        let stage2 = Hkdf::<Sha256>::new(Some(&self.salt), &ikm);
        let mut cek = [0u8; 16];
        stage2
            .expand(b"Content-Encoding: aes128gcm\0", &mut cek)
            .map_err(|e| anyhow::anyhow!("HKDF expand for CEK failed: {e}"))?;
        let mut nonce = [0u8; 12];
        stage2
            .expand(b"Content-Encoding: nonce\0", &mut nonce)
            .map_err(|e| anyhow::anyhow!("HKDF expand for nonce failed: {e}"))?;

        // Single-record plaintext per RFC 8188 §2: `message || 0x02`. The
        // 0x02 byte is the last-record delimiter; no extra padding bytes are
        // added (push services don't care, and shorter is cheaper).
        let mut plaintext = Vec::with_capacity(message.len() + 1);
        plaintext.extend_from_slice(message);
        plaintext.push(0x02);

        let cipher = Aes128Gcm::new(Key::<Aes128Gcm>::from_slice(&cek));
        let ciphertext = cipher
            .encrypt(Nonce::from_slice(&nonce), plaintext.as_ref())
            .map_err(|e| anyhow::anyhow!("AES-GCM encrypt failed: {e}"))?;

        // RFC 8188 §2.1 framing header followed by the ciphertext.
        let mut body =
            Vec::with_capacity(SALT_LEN + 4 + 1 + PUBLIC_KEY_LEN + ciphertext.len());
        body.extend_from_slice(&self.salt);
        body.extend_from_slice(&RECORD_SIZE.to_be_bytes());
        body.push(PUBLIC_KEY_LEN as u8);
        body.extend_from_slice(&server_public);
        body.extend_from_slice(&ciphertext);

        Ok(Encrypted {
            body,
            server_public_key: server_public,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use base64::prelude::*;

    fn b64u(s: &str) -> Vec<u8> {
        BASE64_URL_SAFE_NO_PAD.decode(s).expect("invalid base64url test vector")
    }

    /// RFC 8291 Appendix A: end-to-end test of the aes128gcm Web Push
    /// content-encoding with the canonical fixed inputs from the spec.
    /// If our encrypt path passes this, the bytes a real push service sees
    /// are correct.
    #[test]
    fn encrypts_rfc_8291_appendix_a_vectors() {
        let plaintext = b"When I grow up, I want to be a watermelon";

        // Application server (sender) keypair from Appendix A.2.
        let as_private: [u8; 32] = b64u("yfWPiYE-n46HLnH0KqZOF1fJJU3MYrct3AELtAQ-oRw")
            .try_into()
            .unwrap();

        // User agent (receiver) public key from Appendix A.2.
        let ua_public: [u8; 65] = b64u(
            "BCVxsr7N_eNgVRqvHtD0zTZsEc6-VV-JvLexhqUzORcxaOzi6-AYWXvTBHm4bjyPjs7Vd8pZGH6SRpkNtoIAiw4",
        )
        .try_into()
        .unwrap();

        // Auth secret and salt also from Appendix A.2.
        let auth: [u8; 16] = b64u("BTBZMqHH6r4Tts7J_aSIgg").try_into().unwrap();
        let salt: [u8; 16] = b64u("DGv6ra1nlYgDCS1FRnbzlw").try_into().unwrap();

        let expected_body = b64u(
            "DGv6ra1nlYgDCS1FRnbzlwAAEABBBP4z9KsN6nGRTbVYI_c7VJSPQTBtkgcy27mlmlMoZIIgDll6e3vCYLocInmYWAmS6TlzAC8wEqKK6PBru3jl7A_yl95bQpu6cVPTpK4Mqgkf1CXztLVBSt2Ks3oZwbuwXPXLWyouBWLVWGNWQexSgSxsj_Qulcy4a-fN",
        );

        let sender = Sender::from_raw(as_private, salt).unwrap();
        let encrypted = sender.encrypt(&ua_public, &auth, plaintext).unwrap();

        assert_eq!(encrypted.body, expected_body, "encrypted body mismatch");

        // The expected sender public key is encoded as the `keyid` portion of
        // the framing header; confirm we return it separately, too.
        let expected_as_public: [u8; 65] = b64u(
            "BP4z9KsN6nGRTbVYI_c7VJSPQTBtkgcy27mlmlMoZIIgDll6e3vCYLocInmYWAmS6TlzAC8wEqKK6PBru3jl7A8",
        )
        .try_into()
        .unwrap();
        assert_eq!(encrypted.server_public_key, expected_as_public);
    }

    /// Sanity check: a freshly generated sender should produce a body of the
    /// right shape (header + ciphertext at minimum tag length) without
    /// panicking.
    #[test]
    fn random_sender_produces_well_shaped_body() {
        let sender = Sender::random();
        // We need a real subscription key for ECDH not to fail; reuse the
        // RFC vector's UA key.
        let ua_public: [u8; 65] = b64u(
            "BCVxsr7N_eNgVRqvHtD0zTZsEc6-VV-JvLexhqUzORcxaOzi6-AYWXvTBHm4bjyPjs7Vd8pZGH6SRpkNtoIAiw4",
        )
        .try_into()
        .unwrap();
        let auth: [u8; 16] = b64u("BTBZMqHH6r4Tts7J_aSIgg").try_into().unwrap();

        let encrypted = sender.encrypt(&ua_public, &auth, b"hello").unwrap();
        // Header: 16 salt + 4 rs + 1 idlen + 65 keyid = 86 bytes.
        // Ciphertext: 6 plaintext bytes (5 + 0x02) + 16 GCM tag = 22 bytes.
        assert_eq!(encrypted.body.len(), 86 + 22);
    }
}
