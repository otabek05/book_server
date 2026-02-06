
use axum::{Json, extract::State, http::StatusCode};
use crate::{
    app_state::AppState,  models::book::{Book, CreateBook}, pkg::api_response::ApiResponse
};


pub async fn list(
    State(state): State<AppState>,
) -> Json<ApiResponse<Vec<Book>>> {
    let books = state.repo.book.find_all().await;
    Json(ApiResponse::success(books, "books receivied successfully"))
}

pub async fn save(
    State(state): State<AppState>,
    Json(dto): Json<CreateBook>,
) -> Json<ApiResponse<Book>>{
    match state.repo.book.save(&dto).await {
        Ok(book) => Json(ApiResponse::success(book, "successfully created")),
        Err(err) => Json(ApiResponse::error(StatusCode::INTERNAL_SERVER_ERROR, err))
    }
}
