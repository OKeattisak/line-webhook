use std::sync::Arc;
use anyhow::Result;
use async_trait::async_trait;
use mockall::automock;

use crate::{domain::{entities::line_channels::{AddLineChannelEntity, UpdateLineChannelEntity}, repositories::line_channels::LineChannelsRepository}, infrastructure::postgres::postgres_connection::PgPool};

pub struct LineChannelsPostgres {
    db_pool: Arc<PgPool>
}

impl LineChannelsPostgres {
    pub fn new(db_pool: Arc<PgPool>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
#[automock]
impl LineChannelsRepository for LineChannelsPostgres {
    async fn add(&self, add_line_channel_entity: AddLineChannelEntity) -> Result<i32> {
        unimplemented!()
    }
    async fn update(&self, id:i32, update_line_channel_entity: UpdateLineChannelEntity) -> Result<i32> {
        unimplemented!()
    }
    async fn delete(&self, id:i32) -> Result<i32> {
        unimplemented!()
    }
}