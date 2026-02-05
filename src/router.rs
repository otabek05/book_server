
use axum::{Router, routing::{get}};
use crate::controllers::book_controller::{self};
use crate::controllers::author_controller::{self};
use crate::app_state::AppState;


pub struct RouteHandler {}


impl RouteHandler {
    pub fn new() -> RouteHandler {
        RouteHandler {  }
    }

    pub fn routes(&self) -> Router<AppState> {
        Router::new()
        .merge(self.book_routes())
        .merge(self.author_routes())
    }

    fn book_routes(&self) -> Router<AppState> {
        Router::new()
            .route("/books", get(book_controller::list).post(book_controller::save))
    }

    fn author_routes(&self) -> Router<AppState> {
        Router::new()
           .route("/authors", get(author_controller::list).post(author_controller::save))
           .route("/authors/:id", get(author_controller::get))
    }
}
