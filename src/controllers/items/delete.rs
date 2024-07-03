use axum::extract::{Path, State};

use crate::{
    api_responses::{ApiError, ApiResponse},
    bootstrap::AppState,
    services::ItemsService,
};

pub async fn delete_item(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<ApiResponse, ApiError> {
    state.items_service.delete_item(id).await.map_err(|e| {
        eprint!("There was an error deleting the item: {e:?}");
        ApiError::UnknownError
    })?;

    Ok(ApiResponse::NoContent)
}
