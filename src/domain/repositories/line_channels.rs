use crate::domain::{
    entities::line_channels::{AddLineChannelEntity, UpdateLineChannelEntity},
};
use anyhow::Result;
use async_trait::async_trait;
use mockall::automock;

#[async_trait]
#[automock]
pub trait LineChannelsRepository {
    async fn add(&self, add_line_channel_entity: AddLineChannelEntity) -> Result<i32>;
    async fn update(&self, id:i32, update_line_channel_entity: UpdateLineChannelEntity) -> Result<i32>;
    async fn delete(&self, id:i32) -> Result<i32>;
}
