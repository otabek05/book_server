use std::{ sync::Arc};

use crate::repo::{author::*, book::*};
use sqlx::MySqlPool;
#[derive(Clone)]
pub struct AppState {
    pub book_repo: Arc< BookRepo>,
    pub author_repo: Arc<AuthorRepo>,
}

impl AppState {
    pub fn new(db: MySqlPool) -> Self {
        AppState { 
            book_repo: Arc::new( BookRepo::new(db.clone())), 
            author_repo:Arc::new(AuthorRepo::new(db.clone())) 
        }
    }
}
