use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct CreateItemRequest {
    pub item_name: String,
}
