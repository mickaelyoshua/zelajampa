use crate::api::handlers::{create_report, get_clusters, index};
use crate::api::state::AppState;
use axum::{
    Router,
    routing::{get, post},
};
use tower_http::services::ServeDir;

pub fn app_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(index))
        .route("/reports", post(create_report))
        .route("/clusters", get(get_clusters))
        .nest_service("/static", ServeDir::new("static"))
        .with_state(state)
}
