//! Announcement fan-out.
//!
//! Delivering an announcement means one HTTPS POST per subscriber, and a Worker invocation gets
//! roughly 1,000 subrequests. A convention large enough to want push notifications has more
//! subscribers than that, so the fan-out cannot happen in the request that receives the webhook.
//!
//! Instead the webhook lists subscription ids — cheap, one `list` per 1,000 — splits them into
//! chunks, and puts each chunk on a queue. Every queue consumer invocation is a *fresh* Worker
//! invocation with its own subrequest and CPU budget, so the work fits by construction however many
//! subscribers there are. Chunks are dispatched together and run in parallel.

use futures::StreamExt;
use serde::{Deserialize, Serialize};
use worker::{Queue, console_warn, kv::KvStore};

use crate::env::EnvName;
use crate::kv;
use crate::push::client::{Client, DeliveryOutcome, SubscriptionId};

/// How many subscriptions one queue message covers.
///
/// Sized against the ~1,000 subrequest budget of the consumer invocation that will handle it. Worst
/// case each subscription costs three: a KV read to resolve it, the POST to the push service, and
/// a KV delete if the push service reports the subscription is gone. 200 × 3 = 600 leaves
/// comfortable headroom.
const CHUNK_SIZE: usize = 200;

/// How many pushes within a chunk are in flight at once.
///
/// Sending serially would take ~30s per chunk at typical push-service latency, which wastes the
/// invocation's wall clock for no reason. The cap keeps us from opening 200 sockets at once.
const SEND_CONCURRENCY: usize = 32;

/// How many messages to put on the queue per `send_batch` call.
///
/// Queues caps a batch send at 100 messages and ~256KB total. At ~7KB per message, 25 stays clear
/// of both without having to account for message sizes at runtime.
const MESSAGES_PER_BATCH: usize = 25;

/// One chunk of an announcement fan-out: the subscribers to notify, and what to send them.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PushJob {
    pub env_name: String,

    /// The serialized `Payload`, sent verbatim to every subscriber in the chunk. Kept as a string
    /// rather than the struct because `Payload` borrows its `title`.
    pub payload: String,

    /// Ids rather than whole subscriptions: resolving them is the consumer's job, and it keeps a
    /// message small enough that batching them is trivial.
    pub subscription_ids: Vec<SubscriptionId>,
}

/// Split a subscriber list into queue-sized units of work. Pure, so the chunking is testable
/// without a Workers runtime.
pub fn chunk_jobs(env_name: &EnvName, payload: &str, ids: Vec<SubscriptionId>) -> Vec<PushJob> {
    ids.chunks(CHUNK_SIZE)
        .map(|chunk| PushJob {
            env_name: env_name.to_string(),
            payload: payload.to_string(),
            subscription_ids: chunk.to_vec(),
        })
        .collect()
}

/// Hand every chunk to the queue. Returns once they're durably accepted; the delivery itself
/// happens in the consumer invocations the queue will spawn.
pub async fn enqueue_jobs(queue: &Queue, jobs: Vec<PushJob>) -> anyhow::Result<()> {
    for batch in jobs.chunks(MESSAGES_PER_BATCH) {
        queue
            .send_batch(batch.to_vec())
            .await
            .map_err(|err| anyhow::anyhow!("failed to enqueue push jobs: {err}"))?;
    }

    Ok(())
}

/// Deliver one chunk. Individual subscribers failing is expected and never fails the job — a dead
/// endpoint shouldn't cost everyone else in the chunk a redelivery.
pub async fn run_job(kv: &KvStore, client: &Client, job: &PushJob) {
    let env_name = EnvName::from(job.env_name.clone());
    let payload = job.payload.as_bytes();

    futures::stream::iter(job.subscription_ids.iter())
        .map(|id| deliver(kv, client, &env_name, id, payload))
        .buffer_unordered(SEND_CONCURRENCY)
        .collect::<Vec<()>>()
        .await;
}

async fn deliver(
    kv: &KvStore,
    client: &Client,
    env_name: &EnvName,
    id: &SubscriptionId,
    payload: &[u8],
) {
    let subscription = match kv::get_subscription(kv, env_name, id).await {
        // The subscriber unsubscribed between being listed and being read. Nothing to do.
        Ok(None) => return,
        Ok(Some(subscription)) => subscription,
        Err(e) => {
            console_warn!("Failed reading subscription {}: {e}", id);
            return;
        }
    };

    match client.send(&subscription, payload).await {
        Ok(DeliveryOutcome::Delivered) => {}
        Ok(DeliveryOutcome::SubscriptionGone) => {
            if let Err(e) = kv::delete_subscription(kv, env_name, id).await {
                console_warn!("Failed evicting dead subscription {}: {e}", id);
            }
        }
        Ok(DeliveryOutcome::OtherStatus(code)) => {
            console_warn!(
                "Push service returned {} for endpoint {}",
                code,
                subscription.endpoint,
            );
        }
        Err(e) => {
            console_warn!(
                "Push send failed for endpoint {}: {e}",
                subscription.endpoint,
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ids(count: usize) -> Vec<SubscriptionId> {
        (0..count)
            .map(|i| SubscriptionId::from(format!("{i:032x}")))
            .collect()
    }

    #[test]
    fn splits_into_chunks_no_larger_than_the_subrequest_budget_allows() {
        let jobs = chunk_jobs(&EnvName::from("mycon".to_string()), "{}", ids(450));

        assert_eq!(jobs.len(), 3);
        assert!(
            jobs.iter()
                .all(|job| job.subscription_ids.len() <= CHUNK_SIZE)
        );
    }

    #[test]
    fn every_subscriber_lands_in_exactly_one_chunk() {
        let all = ids(450);
        let jobs = chunk_jobs(&EnvName::from("mycon".to_string()), "{}", all.clone());

        let fanned: Vec<_> = jobs
            .iter()
            .flat_map(|job| job.subscription_ids.iter().cloned())
            .collect();

        assert_eq!(fanned, all);
    }

    #[test]
    fn no_subscribers_means_no_messages() {
        let jobs = chunk_jobs(&EnvName::from("mycon".to_string()), "{}", ids(0));

        assert!(jobs.is_empty());
    }
}
