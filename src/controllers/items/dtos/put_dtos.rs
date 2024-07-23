use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct UpdateItemRequest {
    pub item_name: String,
}
