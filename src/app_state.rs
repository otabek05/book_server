use std::{ sync::Arc};

use crate::repo::{author::*, book::*, user::UserRepo};
use sqlx::MySqlPool;
#[derive(Clone)]
pub struct AppState {
    pub book_repo: Arc< BookRepo>,
    pub author_repo: Arc<AuthorRepo>,
    pub user_repo: Arc<UserRepo>
}

impl AppState {
    pub fn new(db: MySqlPool) -> Self {
        AppState { 
            book_repo: Arc::new( BookRepo::new(db.clone())), 
            author_repo:Arc::new(AuthorRepo::new(db.clone())) ,
            user_repo: Arc::new(UserRepo::new(db.clone()))
        }
    }
}
