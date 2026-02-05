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

#[tokio::main]
async  fn  main() {
    dotenvy::dotenv().ok();
    let config = config::from_file();
    let pool = create_pool(&config).await;
    let state = AppState::new(pool);
    let router = router::RouteHandler::new();

    let app = Router::new()
        .merge(router.routes())
        .with_state(state.clone());

    let addr = format!("{}:{}", config.server.host, config.server.port);
    let listener = TcpListener::bind(&addr).await.expect("Failed to bind address");

    axum::serve(listener, app).await.unwrap();
}
