use crate::api_responses::{ApiError, ApiResponse};

pub async fn health_check() -> Result<ApiResponse, ApiError> {
    Ok(ApiResponse::OkWithoutBody)
}
