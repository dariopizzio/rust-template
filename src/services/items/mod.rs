mod items_service;
pub use items_service::ItemsServiceImpl;

use crate::{controllers::CreateItemRequest, models::items::Item};

use super::errors::ServiceError;

pub trait ItemsService {
    async fn get_item(&self, id: i32) -> Result<Option<Item>, ServiceError>;

    async fn create_item(&self, item: CreateItemRequest) -> Result<Item, ServiceError>;
}
