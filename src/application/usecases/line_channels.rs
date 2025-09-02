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

    pub async fn add(&self, mut add_line_channel_model: AddLineChannelModel) -> Result<i32> {
        unimplemented!()
    }

    pub async fn update(&self, id: i32, mut update_line_channel_model: UpdateLineChannelModel) -> Result<i32> {
        unimplemented!()
    }

    pub async fn delete(&self, id: i32) -> Result<i32> {
        unimplemented!()
    }
}
