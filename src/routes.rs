use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    bootstrap::AppState,
    controllers::{get_item, post_item},
    health::health_check,
};

pub fn init_router(app_state: AppState) -> Router {
    // TODO add versioning
    Router::new()
        .route("/health", get(health_check))
        .route("/item/:id", get(get_item))
        .route("/item", post(post_item))
        .with_state(app_state)
}
