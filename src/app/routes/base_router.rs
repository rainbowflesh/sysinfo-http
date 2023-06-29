use axum::{routing::get, Router};

use crate::app::controllers::base_controller::{
    handler_404, handler_418, handler_500, health_check, support_check,
};

pub fn create_base_router() -> Router {
    let router = Router::new();
    let base_router = router
        .route("/", get(handler_418))
        .route("/404", get(handler_404))
        .route("/500", get(handler_500))
        .route("/health", get(health_check))
        .route("/support", get(support_check))
        .route("/teapot", get(handler_418));
    let base_router = base_router.fallback(handler_404);
    base_router
}
