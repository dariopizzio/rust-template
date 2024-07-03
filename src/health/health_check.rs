use axum::Json;
use serde_json::json;

use crate::api_responses::{ApiError, ApiResponse};

pub async fn health_check() -> Result<ApiResponse, ApiError> {
    Ok(ApiResponse::Ok(Json(json!(""))))
}
