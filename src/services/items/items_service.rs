use crate::{
    controllers::{CreateItemRequest, UpdateItemRequest},
    models::items::Item,
    repositories::{ItemsRepository, ItemsRepositoryImpl, RepositoryError},
    services::errors::ServiceError,
};

use super::ItemsService;

#[derive(Clone)]
pub struct ItemsServiceImpl {
    items_repository: ItemsRepositoryImpl,
}

// Use generics to avoid coupling the repository
impl ItemsServiceImpl {
    pub fn new(items_repository: ItemsRepositoryImpl) -> Self {
        Self { items_repository }
    }
}

impl ItemsService for ItemsServiceImpl {
    async fn get_item(&self, id: i32) -> Result<Option<Item>, ServiceError> {
        self.items_repository
            .get(id)
            .await
            .map_err(|e: RepositoryError| ServiceError::UnknownError(anyhow::anyhow!(e)))
    }

    async fn get_all_items(&self) -> Result<Vec<Item>, ServiceError> {
        self.items_repository
            .get_all()
            .await
            .map_err(|e: RepositoryError| ServiceError::UnknownError(anyhow::anyhow!(e)))
    }

    async fn create_item(&self, item: CreateItemRequest) -> Result<Item, ServiceError> {
        let item_dto = item.into();

        self.items_repository
            .create(item_dto)
            .await
            .map_err(|e: RepositoryError| ServiceError::UnknownError(anyhow::anyhow!(e)))
    }

    async fn update_item(&self, id: i32, item: UpdateItemRequest) -> Result<(), ServiceError> {
        let item_dto = item.into();

        self.items_repository
            .update(id, item_dto)
            .await
            .map_err(|e: RepositoryError| ServiceError::UnknownError(anyhow::anyhow!(e)))
    }

    async fn delete_item(&self, id: i32) -> Result<(), ServiceError> {
        self.items_repository
            .delete(id)
            .await
            .map_err(|e: RepositoryError| ServiceError::UnknownError(anyhow::anyhow!(e)))
    }
}
