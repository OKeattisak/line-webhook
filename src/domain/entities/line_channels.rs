use chrono::NaiveDateTime;
use diesel::prelude::*;

use crate::infrastructure::postgres::schema::line_channels;

#[derive(Debug, Clone, Identifiable, Selectable, Queryable)]
#[diesel(table_name = line_channels)]
pub struct LineChannelEntity {
    pub id: String,
    pub channel_id: String,
    pub channel_secret: String,
    pub channel_access_token: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
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
