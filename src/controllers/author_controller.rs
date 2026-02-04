
use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode};
use sqlx::MySqlPool;
use crate::{
    app_state::AppState, 
    models::author::{Author, AuthorPayload}, 
    pkg::api_response::ApiResponse, 
    db::author_repo::AuthorRepo,
};

pub struct AuthorController {
    db: MySqlPool,
    author_repo: AuthorRepo
}


impl AuthorController {
    pub fn new(db: MySqlPool) -> Self {
        let author_repo = AuthorRepo::new(db.clone()); 
        AuthorController { author_repo: author_repo, db: db }
    }

    pub async fn save(&self, Json(dto): Json<AuthorPayload>  ) -> Json<ApiResponse<Author>> {
        match self.author_repo.save(&dto).await {
            Ok(author) => Json(ApiResponse::success(author, "author created successfully")),
            Err(err) =>{
                let err_msg = err.to_string();
                eprintln!("db error : {}", err_msg);
                Json(ApiResponse::error(StatusCode::INTERNAL_SERVER_ERROR, err_msg))
            }
        }
    }


    

}