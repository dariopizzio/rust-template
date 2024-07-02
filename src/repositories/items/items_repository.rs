use crate::{models::items::Item, repositories::errors::RepositoryError};

use super::{
    dtos::{CreateItemDto, ItemDto, UpdateItemDto},
    ItemsRepository,
};

use crate::schema::items as itemsSchema;
use crate::schema::items::dsl::items;

use deadpool_diesel::postgres::Object;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};

#[derive(Clone)]
pub struct ItemsRepositoryImpl {
    db_pool: deadpool_diesel::postgres::Pool,
}

impl ItemsRepositoryImpl {
    pub fn new(db_pool: deadpool_diesel::postgres::Pool) -> Self {
        Self { db_pool }
    }

    async fn get_connection(&self) -> Result<Object, RepositoryError> {
        self.db_pool
            .get()
            .await
            .map_err(|e| RepositoryError::DatabaseConnectionError(anyhow::anyhow!(e)))
    }
}

impl ItemsRepository for ItemsRepositoryImpl {
    async fn get(&self, id: i32) -> Result<Option<Item>, RepositoryError> {
        let conn = self.get_connection().await?;

        let result = conn
            .interact(move |conn| items.find(id).select(ItemDto::as_select()).first(conn))
            .await
            .map_err(|_e| RepositoryError::UnknownError)?;

        match result {
            Ok(item) => Ok(Some(item.into())),
            Err(diesel::result::Error::NotFound) => Ok(None),
            Err(e) => Err(RepositoryError::DatabaseError(anyhow::anyhow!(e))),
        }
    }

    async fn create(&self, item: CreateItemDto) -> Result<Item, RepositoryError> {
        let conn = self.get_connection().await?;

        let result = conn
            .interact(move |conn| {
                diesel::insert_into(itemsSchema::table)
                    .values(item)
                    .returning(ItemDto::as_returning())
                    .get_result(conn)
            })
            .await
            .map_err(|_e| RepositoryError::UnknownError)?
            .map_err(|e| RepositoryError::DatabaseError(anyhow::anyhow!(e)))?;

        Ok(result.into())
    }

    async fn update(&self, item: UpdateItemDto) -> Result<(), RepositoryError> {
        let conn = self.get_connection().await?;

        conn.interact(move |conn| {
            diesel::update(items.find(item.id))
                .set(itemsSchema::item_name.eq(item.item_name))
                .execute(conn)
        })
        .await
        .map_err(|_e| RepositoryError::UnknownError)?
        .map_err(|e| RepositoryError::DatabaseError(anyhow::anyhow!(e)))?;

        Ok(())
    }

    async fn delete(&self, id: i32) -> Result<(), RepositoryError> {
        let conn = self.get_connection().await?;

        conn.interact(move |conn| {
            diesel::delete(items.filter(itemsSchema::id.eq(id))).execute(conn)
        })
        .await
        .map_err(|_e| RepositoryError::UnknownError)?
        .map_err(|e| RepositoryError::DatabaseError(anyhow::anyhow!(e)))?;

        Ok(())
    }
}
