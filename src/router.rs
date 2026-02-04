
use axum::{Router, routing::{get}};
use sqlx::MySqlPool;
use crate::controllers::{author_controller::AuthorController, *}
use crate::app_state::AppState;


pub struct RouteHandler {
    author_controller: AuthorController,
}


impl RouteHandler {
    pub fn new(db : MySqlPool) -> RouteHandler {
        let author_controller = AuthorController::new(db);
        RouteHandler { author_controller: author_controller }
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
