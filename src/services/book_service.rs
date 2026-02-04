use axum::http::StatusCode;
use sqlx::MySqlPool;
use crate::db::book_repo;
use crate::models::{Book, CreateBook};

use crate::pkg::api_response::ApiResponse;

pub async fn list_books(db: &MySqlPool) -> ApiResponse<Vec<Book>> {
    match book_repo::find_all(db).await {
     books => ApiResponse::success(books, "Books fetched successfully"),
    }
}

pub async fn create(db: &MySqlPool, dto: CreateBook) -> ApiResponse<Book> {
    match book_repo::insert(db, dto).await {
        Ok(book) => ApiResponse::success(book, "Book created successfully"),
        Err(err) => {
            let err_msg = err.to_string();
            eprintln!("Failed to insert book: {:?}", err );
            ApiResponse::error(StatusCode::INTERNAL_SERVER_ERROR, err_msg)
        }
    }
}