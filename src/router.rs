
use axum::{Router, routing::{get}};
use crate::controllers::book::{self};
use crate::controllers::author::{self};
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
            .route("/books", get(book::list).post(book::save))
    }

    fn author_routes(&self) -> Router<AppState> {
        Router::new()
           .route("/authors", get(author::list).post(author::save))
           .route("/authors/:id", get(author::get).delete(author::delete))
    }
}
