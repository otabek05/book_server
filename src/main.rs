mod app_state;
mod repo;
mod models;
mod controllers;
mod pkg;
mod router;
mod services;

use axum::{Router};
use app_state::AppState;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]

async  fn  main() {
    dotenvy::dotenv().ok();
    let config = pkg::config::from_file();

    let db = match pkg::mariadb::connect(&config).await {
        Ok(db)  => db,
        Err(err) => panic!("Error connecting to db: {}", err)
    };

    let state = AppState::new(db);
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