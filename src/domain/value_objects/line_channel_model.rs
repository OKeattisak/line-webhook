use serde::{Deserialize, Serialize};

use crate::domain::entities::line_channels::{AddLineChannelEntity, UpdateLineChannelEntity};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddLineChannelModel {
    pub channel_id: String,
    pub channel_secret: String,
    pub channel_access_token: String,
}

impl AddLineChannelModel {
    pub fn to_entity(&self) -> AddLineChannelEntity {
        AddLineChannelEntity {
            channel_id: self.channel_id.clone(),
            channel_secret: self.channel_secret.clone(),
            channel_access_token: self.channel_access_token.clone(),
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateLineChannelModel {
    pub channel_secret: Option<String>,
    pub channel_access_token: Option<String>,
}

impl UpdateLineChannelModel {
    pub fn to_entity(&self) -> UpdateLineChannelEntity {
        UpdateLineChannelEntity {
            channel_secret: self.channel_secret.clone(),
            channel_access_token: self.channel_access_token.clone(),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }
}
