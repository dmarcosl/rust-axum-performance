use crate::domain::domains::Domain;
use crate::domain::repositories::ModelRepository;
use crate::domain::services::Service;
use crate::infrastructure::database::adapter::ModelRepositoryImpl;
use axum::async_trait;

pub struct UseCase {
    repo: ModelRepositoryImpl,
}

impl UseCase {
    pub async fn new() -> Self {
        Self {
            repo: ModelRepositoryImpl::new().await,
        }
    }
}

#[async_trait]
impl Service for UseCase {
    async fn post_item(&self, domain: Domain) -> Domain {
        self.repo.insert_item(domain).await.unwrap()
    }

    async fn get_item(&self, item_id: i64) -> Domain {
        self.repo.find_item_by_id(item_id).await.unwrap()
    }

    async fn get_items(&self) -> Vec<Domain> {
        self.repo.find_all_items().await.unwrap()
    }
}
