use serde::{Deserialize, Serialize};


#[derive(Serialize)]
pub struct Book {
    pub id: i64,
    pub title: String,
    pub author_id: i64,
    pub price: f64,
    pub stock: i32,
}

#[derive(Deserialize)]
pub struct CreateBook {
    pub title: String,
    pub author_id: i64,
    pub price: f64,
    pub stock: i32,
}


