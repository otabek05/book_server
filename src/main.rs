mod app_state;
mod config;
mod db;
mod models;
mod controllers;
mod pkg;
mod router;

use axum::{Router};
use app_state::AppState;
use db::create_pool;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]

async  fn  main() {
    dotenvy::dotenv().ok();
    let config = config::from_file();
    let pool = create_pool(&config).await;
    let state = AppState::new(pool);
    let router = router::RouteHandler::new();

    let app = Router::new()
        .merge(router.routes())
        .layer(cors())
        .with_state(state.clone());

    let addr = format!("{}:{}", config.server.host, config.server.port);
    let listener = TcpListener::bind(&addr).await.expect("Failed to bind address");

    axum::serve(listener, app).await.unwrap();
}


fn cors() -> CorsLayer {
    CorsLayer::new()
    .allow_origin(Any)
    .allow_methods(Any)
    .allow_headers(Any)
}