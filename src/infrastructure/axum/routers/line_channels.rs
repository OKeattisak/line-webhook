use std::sync::Arc;

use axum::{
    extract::{Path, State}, http::StatusCode, response::IntoResponse, routing::{delete, patch, post, get}, Json, Router
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
        .route("/", get(handle_getall))
        .route("/{id}", get(handle_get))
        .route("/", post(handle_add))
        .route("/{id}", patch(handle_update))
        .route("/{id}", delete(handle_delete))
        .with_state(Arc::new(line_channels_usecase))
}

pub async fn handle_get<T>(
    State(line_channels_usecase): State<Arc<LineChannelsUseCase<T>>>,
    Path(id): Path<i32>,
) -> impl IntoResponse
where
    T: LineChannelsRepository + Send + Sync,
{
    match line_channels_usecase.get(id).await {
        Ok(line_channel_model) => (StatusCode::OK, Json(line_channel_model)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn handle_getall<T>(
    State(line_channels_usecase): State<Arc<LineChannelsUseCase<T>>>,
) -> impl IntoResponse
where
    T: LineChannelsRepository + Send + Sync,
{
    match line_channels_usecase.get_all().await {
        Ok(line_channel_models) => (StatusCode::OK, Json(line_channel_models)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn handle_add<T>(
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

pub async fn handle_update<T>(
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

pub async fn handle_delete<T>(
    State(line_channels_usecase): State<Arc<LineChannelsUseCase<T>>>,
    Path(id): Path<i32>,
) -> impl IntoResponse
where
    T: LineChannelsRepository + Send + Sync,
{
    match line_channels_usecase.delete(id).await {
        Ok(id) => (
            StatusCode::OK,
            format!("Delete line channel id: {} successfully", id),
        )
            .into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}
