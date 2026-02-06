use crate::app_state::AppState;
use crate::controllers::author::{self};
use crate::controllers::user::{self};
use crate::{
    controllers::book::{self},
    middleware,
};
use axum::{
    Router,
    middleware as axMW,
    routing::{get, post},
};
use tower_http::cors::{Any, CorsLayer};

pub struct RouteHandler {}

impl RouteHandler {
    pub fn new() -> RouteHandler {
        RouteHandler {}
    }

    pub fn router_handler(&self, state: AppState) -> Router {
        Router::new()
            .merge( self.public_routes())
            .nest("/api/v1/private", self.private_routes(state.clone()))
            .layer(self.cors())
            .with_state(state.clone())
    }

    fn public_routes(&self) -> Router<AppState> {
        Router::new().route("/books", get(book::list).post(book::save)).merge(self.user_routes())
    }

    fn private_routes(&self, state: AppState) -> Router<AppState> {
        Router::new()
            .merge( self.author_routes())
          //  .merge(self.author_routes())
            .layer(axMW::from_fn_with_state(
                state.clone(),
                middleware::auth_middleware,
            ))
    }

    fn author_routes(&self) -> Router<AppState> {
        Router::new()
            .route("/authors", get(author::list).post(author::save))
            .route("/authors/:id", get(author::get).delete(author::delete))
    }

    fn user_routes(&self) -> Router<AppState> {
        Router::new()
            .route("/users", post(user::save).get(user::list))
            .route("/users/:id", get(user::get_by_id).delete(user::delete))
            .route("/login", post(user::login))
    }

    fn cors(&self) -> CorsLayer {
        CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any)
    }
}
