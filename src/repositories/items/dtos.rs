use diesel::prelude::*;
use serde::Serialize;

use crate::{
    controllers::{CreateItemRequest, UpdateItemRequest},
    repositories::items::Item,
};

#[derive(Serialize, Queryable, Selectable)]
#[diesel(table_name = crate::schema::items)]
#[diesel(primary_key(id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ItemDto {
    pub id: i32,
    pub item_name: String,
}

impl From<ItemDto> for Item {
    fn from(value: ItemDto) -> Self {
        Item {
            id: value.id,
            item_name: value.item_name,
        }
    }
}

#[derive(Serialize, Insertable)]
#[diesel(table_name = crate::schema::items)]
pub struct CreateItemDto {
    pub id: Option<i32>,
    pub item_name: String,
}

// TODO move
impl From<CreateItemRequest> for CreateItemDto {
    fn from(value: CreateItemRequest) -> Self {
        CreateItemDto {
            id: None,
            item_name: value.item_name,
        }
    }
}

#[derive(Serialize, Insertable)]
#[diesel(table_name = crate::schema::items)]
pub struct UpdateItemDto {
    pub item_name: String,
}

// TODO move
impl From<UpdateItemRequest> for UpdateItemDto {
    fn from(value: UpdateItemRequest) -> Self {
        UpdateItemDto {
            item_name: value.item_name,
        }
    }
}
