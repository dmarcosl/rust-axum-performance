use crate::application::use_case::UseCase;
use crate::infrastructure::web::handlers::routes;
use axum::Router;

pub fn create_app(use_case: UseCase) -> Router {
    Router::new().merge(routes(use_case))
}
