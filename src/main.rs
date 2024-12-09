mod app;
mod application;
mod domain;

mod infrastructure {
    pub mod database;
    pub mod web;
}

use crate::application::use_case::UseCase;

#[tokio::main]
async fn main() {
    let use_case = UseCase::new().await;

    let app = app::create_app(use_case);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
