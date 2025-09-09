use serde::{Deserialize, Serialize};

use crate::domain::entities::line_webhook_events::{
    AddLineWebhookEventEntity, LineWebhookEventEntity,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineWebHookEventModel {
    pub id: i32,
    pub destination: String,
    pub payload: serde_json::Value,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl LineWebHookEventModel {
    pub fn to_entity(&self) -> LineWebhookEventEntity {
        LineWebhookEventEntity {
            id: self.id,
            destination: self.destination.clone(),
            payload: self.payload.clone(),
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddLineWebHookEventModel {
    pub destination: String,
    pub events: serde_json::Value,
}

impl AddLineWebHookEventModel {
    pub fn to_entity(&self) -> AddLineWebhookEventEntity {
        AddLineWebhookEventEntity {
            destination: self.destination.clone(),
            payload: self.events.clone(),
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }
}
