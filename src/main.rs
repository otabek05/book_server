mod app_state;
mod config;
mod db;
mod models;
mod services;
mod controllers;
mod routes;
mod pkg;
mod router;

use std::sync::Arc;

use axum::{Router};
use app_state::AppState;
use db::create_pool;
use routes::book_routes;
use tokio::net::TcpListener;

#[tokio::main]
async  fn  main() {
    
    let config = config::from_file();
    let pool = create_pool(&config).await;

    let state = AppState { db: Arc::new( pool) };

    let app = Router::new()
        .merge(book_routes::routes())
        .with_state(state.clone());

    let addr = format!("{}:{}", config.server.host, config.server.port);
    let listener = TcpListener::bind(&addr).await.expect("Failed to bind address");

    axum::serve(listener, app).await.unwrap();
}
