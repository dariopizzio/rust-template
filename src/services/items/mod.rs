mod items_service;
pub use items_service::ItemsServiceImpl;

use crate::{
    controllers::{CreateItemRequest, UpdateItemRequest},
    models::items::Item,
};

use super::errors::ServiceError;

pub trait ItemsService {
    // TODO check if it's not better to have references as arguments
    async fn get_item(&self, id: i32) -> Result<Option<Item>, ServiceError>;

    async fn get_all_items(&self) -> Result<Vec<Item>, ServiceError>;

    async fn create_item(&self, item: CreateItemRequest) -> Result<Item, ServiceError>;

    async fn update_item(&self, id: i32, item: UpdateItemRequest) -> Result<(), ServiceError>;

    async fn delete_item(&self, id: i32) -> Result<(), ServiceError>;
}
