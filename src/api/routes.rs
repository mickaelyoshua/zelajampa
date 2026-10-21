use crate::api::handlers::{create_report, get_clusters, index};
use crate::api::state::AppState;
use axum::{
    Router,
    routing::{get, post},
};

pub fn app_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(index))
        .route("/reports", post(create_report))
        .route("/clusters", get(get_clusters))
        .with_state(state)
}
