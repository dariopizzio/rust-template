use axum::extract::{Path, State};

use crate::{
    api_responses::{ApiError, ApiResponse},
    bootstrap::AppState,
    services::ItemsService,
};

#[tracing::instrument(skip(state))]
pub async fn delete_item(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<ApiResponse, ApiError> {
    state.items_service.delete_item(id).await.map_err(|e| {
        tracing::error!("There was an error deleting the item with id: {id} - {e:?}");
        ApiError::UnknownError
    })?;

    Ok(ApiResponse::NoContent)
}
