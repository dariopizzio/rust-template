use axum::{extract::State, Json};
use serde_json::json;

use crate::{
    api_responses::{ApiError, ApiResponse},
    bootstrap::AppState,
    services::ItemsService,
};

use super::dtos::GetItemResponse;

pub async fn get_all_items(State(state): State<AppState>) -> Result<ApiResponse, ApiError> {
    let items = state.items_service.get_all_items().await.map_err(|e| {
        eprint!("There was an error getting all items: {e:?}");
        ApiError::UnknownError
    })?;

    let items = items
        .into_iter()
        .map(|item| item.into())
        .collect::<Vec<GetItemResponse>>();

    Ok(ApiResponse::Ok(Json(json!(items))))
}
