use worker::{console_warn, kv::KvStore};

use crate::env::EnvName;
use crate::kv;
use crate::push::client::{Client, DeliveryOutcome};

pub async fn push_notifications(
    kv: &KvStore,
    env_name: &EnvName,
    client: &Client,
    payload: &[u8],
) -> anyhow::Result<()> {
    let subscriptions = kv::list_subscriptions(kv, env_name).await?;
    for stored in subscriptions {
        match client.send(&stored.subscription, payload).await {
            Ok(DeliveryOutcome::Delivered) => {}
            Ok(DeliveryOutcome::SubscriptionGone) => {
                kv::delete_subscription(kv, env_name, &stored.subscription.id()).await?;
            }
            Ok(DeliveryOutcome::OtherStatus(code)) => {
                console_warn!(
                    "Push service returned {} for endpoint {}",
                    code,
                    stored.subscription.endpoint,
                );
            }
            Err(e) => {
                console_warn!(
                    "Push send failed for endpoint {}: {e}",
                    stored.subscription.endpoint,
                );
            }
        }
    }
    Ok(())
}
