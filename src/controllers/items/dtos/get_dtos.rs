use serde::Serialize;

use crate::models::items::Item;

#[derive(Serialize)]
pub struct GetItemResponse {
    id: i32,
    item_name: String,
}

impl From<Item> for GetItemResponse {
    fn from(value: Item) -> Self {
        GetItemResponse {
            id: value.id,
            item_name: value.item_name,
        }
    }
}
