use crate::domain::domains::Domain;
use axum::async_trait;

#[async_trait]
pub trait Service {
    async fn post_item(&self, domain: Domain) -> Domain;
    async fn get_item(&self, item_id: i64) -> Domain;
    async fn get_items(&self) -> Vec<Domain>;
}
