use anyhow::{Ok, Result};
use async_trait::async_trait;
use diesel::{ExpressionMethods, RunQueryDsl, insert_into, update, delete, QueryDsl};
use mockall::automock;
use std::sync::Arc;

use crate::{
    domain::{
        entities::line_channels::{LineChannelEntity, AddLineChannelEntity, UpdateLineChannelEntity},
        repositories::line_channels::LineChannelsRepository,
    },
    infrastructure::postgres::{postgres_connection::PgPool, schema::line_channels},
};

pub struct LineChannelsPostgres {
    db_pool: Arc<PgPool>,
}

impl LineChannelsPostgres {
    pub fn new(db_pool: Arc<PgPool>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
#[automock]
impl LineChannelsRepository for LineChannelsPostgres {

    async fn get(&self, id: i32) -> Result<LineChannelEntity> {
        let mut conn = Arc::clone(&self.db_pool).get()?;

        let result = line_channels::table
            .filter(line_channels::id.eq(id))
            .first::<LineChannelEntity>(&mut conn)?;

        Ok(result)
    }

    async fn get_all(&self) -> Result<Vec<LineChannelEntity>> {
        let mut conn = Arc::clone(&self.db_pool).get()?;
        let results = line_channels::table
            .load::<LineChannelEntity>(&mut conn)?;
        
        Ok(results)
    }

    async fn add(&self, add_line_channel_entity: AddLineChannelEntity) -> Result<i32> {
        let mut conn = Arc::clone(&self.db_pool).get()?;
        let result = insert_into(line_channels::table)
            .values(add_line_channel_entity)
            .returning(line_channels::id)
            .get_result::<i32>(&mut conn)?;

        Ok(result)
    }

    async fn update(
        &self,
        id: i32,
        update_line_channel_entity: UpdateLineChannelEntity,
    ) -> Result<i32> {
        let mut conn = Arc::clone(&self.db_pool).get()?;

        let result = update(line_channels::table)
            .filter(line_channels::id.eq(id))
            .set(update_line_channel_entity)
            .returning(line_channels::id)
            .get_result::<i32>(&mut conn)?;
        Ok(result)
    }

    async fn delete(&self, id: i32) -> Result<i32> {
        let mut conn = Arc::clone(&self.db_pool).get()?;

        let result = delete(line_channels::table)
            .filter(line_channels::id.eq(id))
            .returning(line_channels::id)
            .get_result::<i32>(&mut conn)?;

        Ok(result)
    }
}
