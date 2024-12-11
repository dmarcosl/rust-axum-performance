use crate::domain::domains::Domain;
use crate::domain::repositories::ModelRepository;
use crate::infrastructure::database::config::DatabaseConfig;
use crate::infrastructure::database::models::Model;
use axum::async_trait;
use sqlx::{query, SqlitePool};

pub struct ModelRepositoryImpl {
    pool: SqlitePool,
}

impl ModelRepositoryImpl {
    pub async fn new() -> Self {
        let pool = DatabaseConfig::create_pool()
            .await
            .expect("Failed to create pool");
        DatabaseConfig::run_migrations(pool.clone()).await;

        Self { pool: pool }
    }
}

#[async_trait]
impl ModelRepository for ModelRepositoryImpl {
    async fn insert_item(&self, domain: Domain) -> Result<Domain, sqlx::Error> {
        let model: Model = domain.into();
        // let model: Model = Model::from(domain);

        let result = query!(
            "INSERT INTO models (name, description, price, quantity, stock, category, url, image_url, is_active)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
             model.name,
             model.description,
             model.price,
             model.quantity,
             model.stock,
             model.category,
             model.url,
             model.image_url,
             model.is_active
        )
            .execute(&self.pool)
            .await?;

        self.find_item_by_id(result.last_insert_rowid()).await
    }

    async fn find_item_by_id(&self, item_id: i64) -> Result<Domain, sqlx::Error> {
        let row = query!(
            "SELECT item_id, name, description, price, quantity, stock, category, url, image_url, is_active
            FROM models
            WHERE item_id = ?",
            item_id
        )
            .fetch_optional(&self.pool)
            .await?
            .unwrap();

        let model = Model {
            item_id: row.item_id,
            name: row.name,
            description: row.description,
            price: row.price as f32,
            quantity: row.quantity,
            stock: row.stock,
            category: row.category,
            url: row.url,
            image_url: row.image_url,
            is_active: row.is_active != 0,
        };

        Ok(model.into())
    }

    async fn find_all_items(&self) -> Result<Vec<Domain>, sqlx::Error> {
        let rows = query!(
            "SELECT item_id, name, description, price, quantity, stock, category, url, image_url, is_active
            FROM models
            LIMIT 10000"
        )
            .fetch_all(&self.pool)
            .await?;

        Ok(rows
            .into_iter()
            .map(|r| {
                let model = Model {
                    item_id: r.item_id,
                    name: r.name,
                    description: r.description,
                    price: r.price as f32,
                    quantity: r.quantity,
                    stock: r.stock,
                    category: r.category,
                    url: r.url,
                    image_url: r.image_url,
                    is_active: r.is_active != 0,
                };
                model.into()
            })
            .collect())
    }
}
