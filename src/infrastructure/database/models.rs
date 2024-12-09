use serde_derive::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct Model {
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
