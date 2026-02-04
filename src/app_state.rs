
use std::sync::Arc;

use sqlx::MySqlPool;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc< MySqlPool>,
}


