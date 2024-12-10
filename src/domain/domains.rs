#[derive(Debug, Clone)]
pub struct Domain {
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
