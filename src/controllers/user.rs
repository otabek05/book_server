use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode, response::IntoResponse,
};

use crate::{
    app_state::AppState,
    models::{CreateUser, Login, Token, User},
    pkg::api_response::ApiResponse,
};

pub async fn save(
    State(state): State<AppState>,
    Json(payload): Json<CreateUser>,
) -> Json<ApiResponse<User>> {
    match state.repo.user.save(payload).await {
        Ok(Some(user)) => Json(ApiResponse::success(user, "successfully created")),
        Ok(None) => Json(ApiResponse::generic_error(
            StatusCode::BAD_REQUEST,
            String::from("user cannot be created"),
        )),
        Err(err) => Json(ApiResponse::error(StatusCode::INTERNAL_SERVER_ERROR, err)),
    }
}

pub async fn list(State(state): State<AppState>) -> Json<ApiResponse<Vec<User>>> {
    match state.repo.user.list().await {
        Ok(users) => Json(ApiResponse::success(users, "success")),
        Err(err) => Json(ApiResponse::error(StatusCode::INTERNAL_SERVER_ERROR, err)),
    }
}

pub async fn get_by_id(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Json<ApiResponse<User>> {
    match state.repo.user.get_by_id(id).await {
        Ok(Some(user)) => Json(ApiResponse::success(user, "success")),
        Ok(None) => Json(ApiResponse::generic_error(
            StatusCode::NOT_FOUND,
            "user not found".to_string(),
        )),
        Err(err) => Json(ApiResponse::error(StatusCode::INTERNAL_SERVER_ERROR, err)),
    }
}

pub async fn delete(State(state): State<AppState>, Path(id): Path<u64>) -> Json<ApiResponse<()>> {
    match state.repo.user.delete(id).await {
        Ok(true) => Json(ApiResponse::success((), "user deleted")),
        Ok(false) => Json(ApiResponse::generic_error(
            StatusCode::NOT_FOUND,
            "user not found".to_string(),
        )),
        Err(err) => Json(ApiResponse::error(StatusCode::INTERNAL_SERVER_ERROR, err)),
    }
}

pub async fn login(
    State(state): State<AppState>,
    Json(login): Json<Login>,
) ->  impl IntoResponse {
    if login.username.trim().is_empty() {
        return Json(ApiResponse::generic_error(
            StatusCode::BAD_REQUEST,
            "email is required".to_string(),
        ));
    }

    let user = match state.repo.user.get_by_email(&login.username).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            return Json(ApiResponse::generic_error(
                StatusCode::BAD_REQUEST,
                "invalid email or password".to_string(),
            ))
        }
        Err(err) => {
            return Json(ApiResponse::error(
                StatusCode::INTERNAL_SERVER_ERROR,
                err,
            ))
        }
    };

     if login.password != "secret" {
        return Json(ApiResponse::generic_error(
            StatusCode::BAD_REQUEST,
            "invalid email or password".to_string(),
        ));
    };

    let token = match  state.service.jwt.generate(user.id) {
        Ok(token) => token,
        Err(err) => return Json(ApiResponse::generic_error(StatusCode::BAD_REQUEST, err.to_string())),
    };

    Json(ApiResponse::success(token, "token successfully created"))
}
