use axum::{
    routing::{delete, get, post, put},
    Router,
};

use crate::{
    bootstrap::AppState,
    controllers::{delete_item, get_all_items, get_item, post_item, put_item},
    health::health_check,
};

pub fn init_router(app_state: AppState) -> Router {
    // TODO add versioning
    Router::new()
        .route("/health", get(health_check))
        .route("/item/:id", get(get_item))
        .route("/item", get(get_all_items))
        .route("/item", post(post_item))
        .route("/item/:id/name", put(put_item))
        .route("/item/:id", delete(delete_item))
        .with_state(app_state)
}
