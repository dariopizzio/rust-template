mod dtos;
mod items_repository;

use dtos::{CreateItemDto, UpdateItemDto};
pub use items_repository::ItemsRepositoryImpl;

use super::errors::RepositoryError;
use crate::models::items::Item;

pub trait ItemsRepository {
    // TODO get-all
    async fn get(&self, id: i32) -> Result<Option<Item>, RepositoryError>;

    async fn create(&self, item: CreateItemDto) -> Result<Item, RepositoryError>;

    async fn update(&self, item: UpdateItemDto) -> Result<(), RepositoryError>;

    async fn delete(&self, id: i32) -> Result<(), RepositoryError>;
}
