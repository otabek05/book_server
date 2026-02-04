
use axum::{Json, extract::State};
use crate::{
    app_state::AppState, 
    models::book::{Book, CreateBook}, 
    pkg::api_response::ApiResponse, 
    services::book_service,
};


pub async fn list_books(
    State(state): State<AppState>,
) -> Json<ApiResponse<Vec<Book>>> {
    Json(book_service::list_books(&state.db).await)
}

pub async fn create_book(
    State(state): State<AppState>,
    Json(dto): Json<CreateBook>,
) -> Json<ApiResponse<Book>>{
    let book = book_service::create(&state.db, dto).await;
    Json(book)
}