use axum::{Json, extract::State, http::StatusCode};

use crate::{app_state::AppState, models::{CreateUser, User}, pkg::api_response::ApiResponse};



pub async fn save(State(state): State<AppState>, Json(payload): Json<CreateUser>) -> Json<ApiResponse<User>> {
    match state.user_repo.save(payload).await {
        Ok(Some(user)) => Json(ApiResponse::success(user, "successfully created")),
        Ok(None) => Json(ApiResponse::generic_error(StatusCode::BAD_REQUEST, String::from("user cannot be created"))),
        Err(err) => Json(ApiResponse::error(StatusCode::INTERNAL_SERVER_ERROR, err)) 
    }
}


