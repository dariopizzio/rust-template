use crate::{repositories::ItemsRepositoryImpl, services::ItemsServiceImpl};

#[derive(Clone)]
pub struct AppState {
    pub items_service: ItemsServiceImpl,
}

pub fn get_app_state(
    // TODO make it generic
    db_pool: deadpool_diesel::Pool<deadpool_diesel::Manager<diesel::PgConnection>>,
) -> AppState {
    let items_repository = ItemsRepositoryImpl::new(db_pool);

    let items_service = ItemsServiceImpl::new(items_repository);

    AppState { items_service }
}
