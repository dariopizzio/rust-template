use axum::{
    body::Body,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::Value;

pub enum ApiResponse {
    Ok(Json<Value>),
    OkWithoutBody,
    Created(String),
}

pub enum ApiError {
    NotFound,
    UnknownError,
}

impl IntoResponse for ApiResponse {
    fn into_response(self) -> Response {
        match self {
            Self::Ok(data) => (StatusCode::OK, data).into_response(),
            Self::OkWithoutBody => (StatusCode::OK).into_response(),
            Self::Created(resource_location) => Response::builder()
                .status(204)
                .header("Location", resource_location)
                .body(Body::empty())
                .unwrap(), // TODO refactor
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::UnknownError => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
            ApiError::NotFound => (StatusCode::NOT_FOUND).into_response(),
        }
    }
}
