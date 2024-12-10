use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Dto {
    pub item_id: i64,
    pub name: String,
    pub description: String,
    pub price: f32,
    pub quantity: i64,
    pub stock: i64,
    pub category: String,
    pub url: String,
    pub image_url: String,
    pub is_active: bool,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DtoPost {
    pub name: String,
    pub description: String,
    pub price: f32,
    pub quantity: i64,
    pub stock: i64,
    pub category: String,
    pub url: String,
    pub image_url: String,
    pub is_active: bool,
}
