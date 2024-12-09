use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Dto {
    pub item_id: i64,
    pub name: String,
    pub description: String,
    pub price: u32,
    pub quantity: i64,
    pub stock: i64,
    pub category: String,
    pub url: String,
    pub image_url: String,
    pub is_active: bool,
}
