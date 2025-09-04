use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{patch, post},
};

use crate::{
    application::usecases::line_channels::LineChannelsUseCase,
    domain::{
        repositories::line_channels::LineChannelsRepository,
        value_objects::line_channel_model::{AddLineChannelModel, UpdateLineChannelModel},
    },
    infrastructure::postgres::{
        postgres_connection::PgPool, repositories::line_channels::LineChannelsPostgres,
    },
};

pub fn routers(db_pool: Arc<PgPool>) -> Router {
    let line_channels_repository = LineChannelsPostgres::new(db_pool);
    let line_channels_usecase = LineChannelsUseCase::new(Arc::new(line_channels_repository));
    Router::new()
        .route("/", post(add))
        .route("/{id}", patch(update))
        .with_state(Arc::new(line_channels_usecase))
}

pub async fn add<T>(
    State(line_channels_usecase): State<Arc<LineChannelsUseCase<T>>>,
    Json(add_line_channel_model): Json<AddLineChannelModel>,
) -> impl IntoResponse
where
    T: LineChannelsRepository + Send + Sync,
{
    match line_channels_usecase.add(add_line_channel_model).await {
        Ok(id) => (
            StatusCode::CREATED,
            format!("Register line channel id: {} successfully", id),
        )
            .into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn update<T>(
    State(line_channels_usecase): State<Arc<LineChannelsUseCase<T>>>,
    Path(id): Path<i32>,
    Json(update_line_channel_model): Json<UpdateLineChannelModel>,
) -> impl IntoResponse
where
    T: LineChannelsRepository + Send + Sync,
{
    match line_channels_usecase
        .update(id, update_line_channel_model)
        .await
    {
        Ok(id) => (
            StatusCode::OK,
            format!("Update line channel id: {} successfully", id),
        )
            .into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}
