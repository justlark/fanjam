//! Web Push client for fanning announcement notifications out to subscribed
//! browsers. Implements just enough of RFC 8291 (message encryption) and
//! RFC 8292 (VAPID JWT) to talk to the major push services (FCM, Mozilla
//! autopush, Apple, Edge) from inside a Cloudflare Worker, where the
//! existing Rust `web-push` crate doesn't compile.

mod client;
mod encrypt;
mod vapid;

#[allow(unused_imports)] // some of these get wired up across slices 2-3
pub use client::{Client, DeliveryOutcome, Subscription, SubscriptionKeys, endpoint_id};
#[allow(unused_imports)] // wired up in slice 2
pub use vapid::VapidKey;
