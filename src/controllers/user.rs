use axum::{Json, extract::{Path, State}, http::StatusCode};

use crate::{app_state::AppState, models::{CreateUser, User}, pkg::api_response::ApiResponse};



pub async fn save(State(state): State<AppState>, Json(payload): Json<CreateUser>) -> Json<ApiResponse<User>> {
    match state.user_repo.save(payload).await {
        Ok(Some(user)) => Json(ApiResponse::success(user, "successfully created")),
        Ok(None) => Json(ApiResponse::generic_error(StatusCode::BAD_REQUEST, String::from("user cannot be created"))),
        Err(err) => Json(ApiResponse::error(StatusCode::INTERNAL_SERVER_ERROR, err)) 
    }
}


pub async fn list(
    State(state): State<AppState>,
) -> Json<ApiResponse<Vec<User>>> {
    match state.user_repo.list().await {
        Ok(users) => Json(ApiResponse::success(users, "success")),
        Err(err) => Json(ApiResponse::error(
            StatusCode::INTERNAL_SERVER_ERROR,
            err,
        )),
    }
}


pub async fn get_by_id(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Json<ApiResponse<User>> {
    match state.user_repo.get_by_id(id).await {
        Ok(Some(user)) => Json(ApiResponse::success(user, "success")),
        Ok(None) => Json(ApiResponse::generic_error(
            StatusCode::NOT_FOUND,
            "user not found".to_string(),
        )),
        Err(err) => Json(ApiResponse::error(
            StatusCode::INTERNAL_SERVER_ERROR,
            err,
        )),
    }
}



pub async fn delete(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Json<ApiResponse<()>> {
    match state.user_repo.delete(id).await {
        Ok(true) => Json(ApiResponse::success((), "user deleted")),
        Ok(false) => Json(ApiResponse::generic_error(
            StatusCode::NOT_FOUND,
            "user not found".to_string(),
        )),
        Err(err) => Json(ApiResponse::error(
            StatusCode::INTERNAL_SERVER_ERROR,
            err,
        )),
    }
}
