use crate::{handlers, types::AppState};
use axum::{Router, routing::get};
use std::sync::Arc;

pub fn file_route_get() -> Router<Arc<AppState>> {
    Router::new()
        .route("/file", get(handlers::aws_file_handler))
        .route("/download", get(handlers::download_handler))
}
