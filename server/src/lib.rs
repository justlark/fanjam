mod api;
mod auth;
mod cache;
mod cf;
mod config;
mod cors;
mod env;
mod error;
mod http;
mod kv;
mod neon;
mod noco;
mod push;
mod router;
mod sql;
mod store;
mod url;

use std::sync::Arc;

use router::AppState;
use tower_service::Service;
use worker::{send::SendWrapper, *};

#[event(fetch)]
async fn fetch(
    req: HttpRequest,
    env: Env,
    ctx: Context,
) -> Result<axum::http::Response<axum::body::Body>> {
    console_error_panic_hook::set_once();

    config::init(&env).expect("failed to initialize config");

    let state = AppState {
        kv: env.kv("KV")?,
        bucket: SendWrapper(env.bucket("ASSETS_BUCKET")?),
        queue: env.queue("PUSH_QUEUE")?,
        ctx: Arc::new(ctx),
    };

    Ok(router::new(state).call(req).await?)
}

// Delivers one chunk of an announcement fan-out. See `push::announce` for why the fan-out is split
// across queue messages rather than done in the webhook request.
#[event(queue)]
async fn queue(batch: MessageBatch<push::PushJob>, env: Env, _ctx: Context) -> Result<()> {
    console_error_panic_hook::set_once();

    // A queue invocation doesn't inherit the fetch handler's isolate, so the config may well be
    // empty here even though the webhook that enqueued this job had it.
    config::init(&env).expect("failed to initialize config");

    let Some(vapid) = config::vapid_key() else {
        // Nothing we can do without a signing key, and retrying won't conjure one. Drop the batch
        // rather than looping it through to the dead-letter queue.
        console_error!("Dropping push jobs: no VAPID key is configured.");
        batch.ack_all();
        return Ok(());
    };

    let kv = env.kv("KV")?;
    let client = push::Client::new(vapid);

    for message in batch.messages()? {
        push::run_job(&kv, &client, message.body()).await;
    }

    batch.ack_all();

    Ok(())
}
