use anyhow::Result;
use std::sync::Arc;

use crate::domain::{
    repositories::line_webhook_events::LineWebhookEventsRepository,
    value_objects::line_webhook_event_model::AddLineWebHookEventModel,
};

pub struct LineWebhookEventsUseCase<T>
where
    T: LineWebhookEventsRepository + Send + Sync,
{
    pub line_webhook_event_repository: Arc<T>,
}

impl<T> LineWebhookEventsUseCase<T>
where
    T: LineWebhookEventsRepository + Send + Sync,
{
    pub fn new(line_webhook_event_repository: Arc<T>) -> Self {
        Self {
            line_webhook_event_repository,
        }
    }

    pub async fn add(
        &self,
        add_line_webhook_events_model: AddLineWebHookEventModel,
    ) -> Result<i32> {
        let add_line_webhook_event_entity = add_line_webhook_events_model.to_entity();
        let id = self
            .line_webhook_event_repository
            .add(add_line_webhook_event_entity)
            .await?;
        Ok(id)
    }
}
