//! Web Push client for fanning announcement notifications out to subscribed
//! browsers. Implements just enough of RFC 8291 (message encryption) and
//! RFC 8292 (VAPID JWT) to talk to the major push services (FCM, Mozilla
//! autopush, Apple, Edge) from inside a Cloudflare Worker, where the
//! existing Rust `web-push` crate doesn't compile.

mod announce;
mod client;
mod encrypt;
mod notification;
mod vapid;

pub use announce::push_notifications;
pub use client::{Client, Subscription, endpoint_id};
pub use notification::{Payload, markdown_to_plain_text};
pub use vapid::VapidKey;
