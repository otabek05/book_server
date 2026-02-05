
use axum::{Json, extract::{Path, State}, http::StatusCode};
use crate::{
    app_state::AppState, models::author::{ Author, AuthorPayload}, pkg::api_response::ApiResponse
};


 pub async fn save(State(state): State<AppState>,  Json(dto): Json<AuthorPayload>  ) -> Json<ApiResponse<Author>> {
        match state.author_repo.save(&dto).await {
            Ok(author) => Json(ApiResponse::success(author, "author created successfully")),
            Err(err) =>  Json(ApiResponse::error(StatusCode::INTERNAL_SERVER_ERROR, err))
        }
}


pub async fn get(State(state): State<AppState>, Path(id): Path<i64>) -> Json<ApiResponse<Author>> {
    match  state.author_repo.get(id).await {
        Ok(Some(author)) => {
            Json(ApiResponse::success(author, "successfully fetched"))
        }

        Ok(None) => {
            Json(ApiResponse::generic_error(
                StatusCode::NOT_FOUND,
                format!("author with id {} not found", id),
            ))
        }

        Err(err) =>  Json(ApiResponse::error(StatusCode::INTERNAL_SERVER_ERROR, err)),

    }
}

pub async  fn list(State(state): State<AppState>) -> Json<ApiResponse<Vec<Author>>> {
     match state.author_repo.find_all().await {
        Ok(data) =>  Json(ApiResponse::success(data, "success")),
        Err(err) => Json(ApiResponse::error(StatusCode::INTERNAL_SERVER_ERROR, err))
     }
}