use crate::application::use_case::UseCase;
use crate::domain::services::Service;
use crate::infrastructure::web::dtos::{Dto, DtoPost};
use axum::extract::State;
use axum::{
    extract::{Json, Path},
    routing::{get, post},
    Router,
};
use axum_response_cache::CacheLayer;
use std::sync::Arc;

async fn health_handler() -> Json<String> {
    Json("OK".to_string())
}

pub async fn post_item_handler(
    State(use_case): State<Arc<UseCase>>,
    Json(body): Json<DtoPost>,
) -> Json<Dto> {
    let domain = body.into();
    let result = use_case.post_item(domain).await;
    Json(result.into())
}

pub async fn get_item_handler(
    State(use_case): State<Arc<UseCase>>,
    Path(item_id): Path<i64>,
) -> Json<Dto> {
    let result = use_case.get_item(item_id).await;
    Json(result.into())
}

async fn get_items_handler(State(use_case): State<Arc<UseCase>>) -> Json<Vec<Dto>> {
    let results = use_case.get_items().await;
    Json(
        results
            .into_iter()
            .map(move |result| result.into())
            .collect(),
    )
}

pub fn routes(use_case: UseCase) -> Router {
    Router::new()
        .route("/health", get(health_handler))
        .route("/post-item", post(post_item_handler))
        .route("/get-item/:item_id", get(get_item_handler))
        .route("/get-item-list", get(get_items_handler))
        .route(
            "/get-item-cache/:item_id",
            get(get_item_handler).layer(CacheLayer::with_lifespan(300)),
        )
        .route(
            "/get-item-cache-list",
            get(get_items_handler).layer(CacheLayer::with_lifespan(300)),
        )
        .with_state(Arc::new(use_case))
    // .with_state(&use_case)
}
