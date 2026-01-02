use ali_graphics_server::types::AppState;
use ali_graphics_server::{get_client_config, routes};
use axum::Router;
use axum::routing::get;
use std::sync::Arc;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let client = get_client_config().await;
    let bucket = "waqas-ali-graphics-846".to_string();
    let app_state = Arc::new(AppState::new(client, bucket));

    println!("App state initialized with AWS S3 client.");
    let app = Router::new()
        .route("/", get(hello))
        .merge(routes::file_route_get())
        .nest_service("/static", ServeDir::new("static"))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn hello() -> &'static str {
    "Hello I am running!"
}
