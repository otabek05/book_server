use crate::models::{Author, AuthorPayload};
use sqlx::MySqlPool;


pub struct AuthorRepo {
    db: MySqlPool
}


impl AuthorRepo {
    pub fn new(db : MySqlPool) -> Self {
        AuthorRepo { db: db }
    }

    pub async fn save(&self, payload: &AuthorPayload) -> Result<Author, sqlx::Error> {
        let rec = sqlx::query!(
            "INSERT INTO authors (name) VALUES (?)",
            payload.name
        )
        .execute(&self.db)
        .await?;

        let id = rec.last_insert_id() as i64;
        Ok(Author {
            id,
            name: payload.name.clone(),
        })
    }

    pub async fn get(&self, id: i64) -> Result<Option<Author>, sqlx::Error> {
        let rec = sqlx::query!(
            "SELECT id, name FROM authors WHERE id = ?",
            id
        )
        .fetch_optional(&self.db)
        .await?;

        Ok(rec.map(|r| Author {
            id: r.id,
            name: r.name,
        }))
    }

    // Find all authors
    pub async fn find_all(&self) -> Result<Vec<Author>, sqlx::Error> {
        let rows = sqlx::query!(
            "SELECT id, name FROM authors"
        )
        .fetch_all(&self.db)
        .await?;

        Ok(rows.into_iter().map(|r| Author {
            id: r.id,
            name: r.name,
        }).collect())
    }

    pub async fn delete(&self, id: i64) -> Result<u64, sqlx::Error> {
        let rec = sqlx::query!(
            "DELETE FROM authors WHERE id = ?",
            id
        )
        .execute(&self.db)
        .await?;

        Ok(rec.rows_affected())
    }
}


