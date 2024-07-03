use serde::Deserialize;

#[derive(Deserialize)]
pub struct UpdateItemRequest {
    pub item_name: String,
}
