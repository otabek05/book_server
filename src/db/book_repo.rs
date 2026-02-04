use crate::models::{Book, CreateBook};
use sqlx::MySqlPool;

pub async fn find_all(db: &MySqlPool) -> Vec<Book> {
    sqlx::query_as!(Book, "SELECT id, title, price, stock, author_id FROM books")
        .fetch_all(db)
        .await
        .unwrap()
}

pub async fn insert(db: &MySqlPool, dto: CreateBook) -> Result<Book, sqlx::Error> {
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
    .execute(db)
    .await?;

    let id = result.last_insert_id() as i64;

    sqlx::query_as!(
        Book,
        r#"
        SELECT id, title, price, stock, author_id
        FROM books WHERE id = ?
        "#,
        id
    )
    .fetch_one(db)
    .await
}
