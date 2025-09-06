use anyhow::Result;
use std::sync::Arc;

use crate::domain::{
    repositories::line_channels::LineChannelsRepository,
    value_objects::line_channel_model::{LineChannelModel, AddLineChannelModel, UpdateLineChannelModel},
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

    pub async fn get(&self, id: i32) -> Result<LineChannelModel> {
        let line_channel_entity = self.line_channel_repository.get(id).await?;
        let line_channel_model = line_channel_entity.to_model();
        Ok(line_channel_model)
    }

    pub async fn get_all(&self) -> Result<Vec<LineChannelModel>> {
        let line_channel_entities = self.line_channel_repository.get_all().await?;
        let line_channel_models = line_channel_entities
            .into_iter()
            .map(|line_channel_entity| line_channel_entity.to_model())
            .collect();
        Ok(line_channel_models)
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
        let id = self.line_channel_repository.delete(id).await?;
        Ok(id)
    }
}
