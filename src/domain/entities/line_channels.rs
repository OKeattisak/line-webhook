use chrono::NaiveDateTime;
use diesel::prelude::*;

use crate::{
    domain::value_objects::line_channel_model::LineChannelModel,
    infrastructure::postgres::schema::line_channels,
};

#[derive(Debug, Clone, Identifiable, Selectable, Queryable)]
#[diesel(table_name = line_channels)]
pub struct LineChannelEntity {
    pub id: i32,
    pub channel_id: String,
    pub channel_secret: String,
    pub channel_access_token: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl LineChannelEntity {
    pub fn to_model(&self) -> LineChannelModel {
        LineChannelModel {
            id: self.id.clone(),
            channel_id: self.channel_id.clone(),
            channel_secret: self.channel_secret.clone(),
            channel_access_token: self.channel_access_token.clone(),
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = line_channels)]
pub struct AddLineChannelEntity {
    pub channel_id: String,
    pub channel_secret: String,
    pub channel_access_token: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, AsChangeset)]
#[diesel(table_name = line_channels)]
pub struct UpdateLineChannelEntity {
    pub channel_secret: Option<String>,
    pub channel_access_token: Option<String>,
    pub updated_at: NaiveDateTime,
}
