use crate::domain::domains::Domain;
use axum::async_trait;

#[async_trait]
pub trait ModelRepository {
    async fn insert_item(&self, domain: Domain) -> Result<Domain, sqlx::Error>;
    async fn find_item_by_id(&self, item_id: i64) -> Result<Domain, sqlx::Error>;
    async fn find_all_items(&self) -> Result<Vec<Domain>, sqlx::Error>;
}
