use axum::{extract::State, Json};

use crate::{
    api_responses::{ApiError, ApiResponse},
    bootstrap::AppState,
    services::ItemsService,
};

use super::dtos::CreateItemRequest;

pub async fn post_item(
    State(state): State<AppState>,
    Json(payload): Json<CreateItemRequest>,
) -> Result<ApiResponse, ApiError> {
    let item = state
        .items_service
        .create_item(payload)
        .await
        .map_err(|e| {
            eprint!("There was an error: {e:?}");
            ApiError::UnknownError
        })?;

    let resource_location = format!("/item/{}", item.id);

    Ok(ApiResponse::Created(resource_location))
}
