// Rotas REST
use axum::{
    routing::get,
    Router,
};

use super::handlers::health;

pub fn create_router() -> axum::Router {

    Router::new()
    .route("/", get(health))
}