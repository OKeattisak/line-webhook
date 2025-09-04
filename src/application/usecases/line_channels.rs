use anyhow::Result;
use std::sync::Arc;

use crate::domain::{
    repositories::line_channels::LineChannelsRepository,
    value_objects::line_channel_model::{AddLineChannelModel, UpdateLineChannelModel},
};

pub struct LineChannelsUseCase<T>
where
    T: LineChannelsRepository + Send + Sync,
{
    pub line_channel_repository: Arc<T>,
}

impl<T> LineChannelsUseCase<T>
where
    T: LineChannelsRepository + Send + Sync,
{
    pub fn new(line_channel_repository: Arc<T>) -> Self {
        Self {
            line_channel_repository,
        }
    }

    pub async fn add(&self, add_line_channel_model: AddLineChannelModel) -> Result<i32> {
        let add_line_channel_entity = add_line_channel_model.to_entity();
        let id = self
            .line_channel_repository
            .add(add_line_channel_entity)
            .await?;
        Ok(id)
    }

    pub async fn update(
        &self,
        id: i32,
        update_line_channel_model: UpdateLineChannelModel,
    ) -> Result<i32> {
        let update_line_channel_entity = update_line_channel_model.to_entity();
        let id = self
            .line_channel_repository
            .update(id, update_line_channel_entity)
            .await?;
        Ok(id)
    }

    pub async fn delete(&self, id: i32) -> Result<i32> {
        unimplemented!()
    }
}
