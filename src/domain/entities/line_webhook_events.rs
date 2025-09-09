use chrono::NaiveDateTime;
use diesel::prelude::*;

use crate::{
    domain::value_objects::line_webhook_event_model::LineWebHookEventModel,
    infrastructure::postgres::schema::line_webhook_events,
};

#[derive(Debug, Clone, Identifiable, Selectable, Queryable)]
#[diesel(table_name = line_webhook_events)]
pub struct LineWebhookEventEntity {
    pub id: i32,
    pub destination: String,
    pub payload: serde_json::Value,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl LineWebhookEventEntity {
    pub fn to_model(&self) -> LineWebHookEventModel {
        LineWebHookEventModel {
            id: self.id.clone(),
            destination: self.destination.clone(),
            payload: self.payload.clone(),
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = line_webhook_events)]
pub struct AddLineWebhookEventEntity {
    pub destination: String,
    pub payload: serde_json::Value,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}