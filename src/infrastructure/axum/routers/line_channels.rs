use std::sync::Arc;

use axum::Router;

use crate::{
    application::usecases::line_channels::LineChannelsUseCase,
    infrastructure::postgres::{
        postgres_connection::PgPool, repositories::line_channels::LineChannelsPostgres,
    },
};

pub fn routers(db_pool: Arc<PgPool>) -> Router {
    let line_channels_repository = LineChannelsPostgres::new(db_pool);
    let line_channels_usecase = LineChannelsUseCase::new(Arc::new(line_channels_repository));
    Router::new().with_state(Arc::new(line_channels_usecase))
}