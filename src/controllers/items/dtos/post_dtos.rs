use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateItemRequest {
    pub item_name: String,
}
