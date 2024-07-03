use axum::{
    extract::{Path, State},
    Json,
};
use serde_json::json;

use crate::{
    api_responses::{ApiError, ApiResponse},
    bootstrap::AppState,
    services::ItemsService,
};

use super::dtos::GetItemResponse;

pub async fn get_item(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<ApiResponse, ApiError> {
    let item = state.items_service.get_item(id).await.map_err(|e| {
        eprint!("There was an error getting the item: {e:?}");
        ApiError::UnknownError
    })?;

    if let Some(item) = item {
        let item: GetItemResponse = item.into();
        Ok(ApiResponse::Ok(Json(json!(item))))
    } else {
        Err(ApiError::NotFound)
    }
}
