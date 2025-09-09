use anyhow::Result;
use async_trait::async_trait;
use mockall::automock;

use crate::domain::entities::line_webhook_events::AddLineWebhookEventEntity;

#[async_trait]
#[automock]
pub trait LineWebhookEventsRepository {
    async fn add(&self, add_line_webhook_event_entity: AddLineWebhookEventEntity) -> Result<i32>;
}
