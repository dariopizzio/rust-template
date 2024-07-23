use axum::{
    extract::{Path, State},
    Json,
};

use crate::{
    api_responses::{ApiError, ApiResponse},
    bootstrap::AppState,
    services::ItemsService,
};

use super::dtos::UpdateItemRequest;

#[tracing::instrument(skip(state))]
pub async fn put_item(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateItemRequest>,
) -> Result<ApiResponse, ApiError> {
    state
        .items_service
        .update_item(id, payload)
        .await
        .map_err(|e| {
            tracing::error!("There was an error updating the item with id: {id} - {e:?}");
            ApiError::UnknownError
        })?;

    Ok(ApiResponse::NoContent)
}
