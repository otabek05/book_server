

use axum::{Router, routing::{get}};
use crate::controllers::book_controller::*;
use crate::app_state::AppState;


pub struct RouteHandler {
    
}


impl RouteHandler {
    pub fn new() -> RouteHandler {
        RouteHandler {  }
    }

    pub fn routes() -> Router<AppState> {
        Router::new()
        .merge(Self::book_routes())
        .merge(Self::author_routes())
    }

    fn book_routes() -> Router<AppState> {
        Router::new()
            .route("/books", get(list_books))
    }

    fn author_routes() -> Router<AppState> {
        Router::new().route("/authors", get(create_book))
    }
}


pub fn routes() -> Router<AppState>{
    Router::new()
        .route("/books", get(list_books).post(create_book))
}
