use crate::models::{Book, CreateBook};
use sqlx::MySqlPool;


pub struct BookRepo {
    db:MySqlPool
}

impl BookRepo {

    pub fn new(db:MySqlPool) -> BookRepo {
        BookRepo { db }
    }    

     pub async fn save(&self, dto: &CreateBook) -> Result<Book, sqlx::Error> {
    let result = sqlx::query!(
        r#"
        INSERT INTO books (title, price, stock, author_id)
        VALUES (?, ?, ?, ?)
        "#,
        dto.title,
        dto.price,
        dto.stock,
        dto.author_id
    )
    .execute(&self.db)
    .await?;

    let id = result.last_insert_id() as i64;

    sqlx::query_as!(
        Book,
        r#"
        SELECT id, title, price, stock, author_id
        FROM books
        WHERE id = ?
        "#,
        id
    )
    .fetch_one(&self.db)
    .await
}


    pub async fn find_all(&self) -> Vec<Book> {
       sqlx::query_as!(
        Book,
        "SELECT id, title, price, stock, author_id FROM books"
    )
    .fetch_all(&self.db)
    .await
    .unwrap()
    }


}
