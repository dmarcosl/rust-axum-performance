extern crate dotenv;

use dotenv::dotenv;
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use std::env;

pub struct DatabaseConfig;

impl DatabaseConfig {
    pub async fn create_pool() -> Result<SqlitePool, sqlx::Error> {
        dotenv().ok();

        let url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
        SqlitePoolOptions::new().connect(&url).await
    }

    pub async fn run_migrations(pool: SqlitePool) {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS models (
                item_id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                description TEXT NOT NULL,
                price INTEGER NOT NULL,
                quantity INTEGER NOT NULL,
                stock INTEGER NOT NULL,
                category TEXT NOT NULL,
                url TEXT NOT NULL,
                image_url TEXT NOT NULL,
                is_active INTEGER NOT NULL
            )",
        )
        .execute(&pool)
        .await
        .expect("Failed to run migrations");
    }
}
