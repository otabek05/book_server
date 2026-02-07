mod app_state;
mod repo;
mod models;
mod controllers;
mod middleware;
mod pkg;
mod router;
mod services;
mod enums;

use app_state::AppState;
use tokio::net::TcpListener;



#[tokio::main]

async  fn  main() {
    dotenvy::dotenv().ok();
    let config = pkg::config::from_file();
    let db = match pkg::mariadb::connect(&config).await {
        Ok(db)  => db,
        Err(err) => panic!("Error connecting to db: {}", err)
    };

    let enforcer = match pkg::enforcer::new(config.middleware.clone()).await {
        Ok(enf) => enf,
        Err(err) => panic!("Error configuring casbin enforcer: {}", err)
    };

    let state = AppState::new(db, &config, enforcer);
    let router = router::RouteHandler::new();
    let addr = format!("{}:{}", config.server.host, config.server.port);
    let listener = TcpListener::bind(&addr).await.expect("Failed to bind address");

    axum::serve(listener, router.router_handler(state)).await.unwrap();
}

