use sqlx::MySqlPool;
use crate::models::{CreateUser, User};




pub struct UserRepo {
    db: MySqlPool
}

impl UserRepo  {
    pub fn new(db:MySqlPool) -> Self {
        UserRepo { db }
    }


     pub async fn save(&self, payload: CreateUser) -> Result<Option<User>, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            INSERT INTO users (name, email, address)
            VALUES (?, ?, ?)
            "#,
            payload.name,
            payload.email,
            payload.address
        )
        .execute(&self.db)
        .await?;

       let user_id = result.last_insert_id();
       let user = self.get_by_id(user_id).await?;
       Ok(user)
    }

    // READ ALL
    pub async fn list(&self) -> Result<Vec<User>, sqlx::Error> {
        let users = sqlx::query_as!(
            User,
            r#"
            SELECT 
                id,
                name,
                email,
                address,
                created_at
            FROM users
            ORDER BY id DESC
            "#
        )
        .fetch_all(&self.db)
        .await?;

        Ok(users)
    }

    // READ BY ID
    pub async fn get_by_id(&self, id: u64) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT
                id,
                name,
                email,
                address,
                created_at
            FROM users
            WHERE id = ?
            "#,
            id
        )
        .fetch_optional(&self.db)
        .await?;

        Ok(user)
    }

    // DELETE
    pub async fn delete(&self, id: u64) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            DELETE FROM users
            WHERE id = ?
            "#,
            id
        )
        .execute(&self.db)
        .await?;

        Ok(result.rows_affected() > 0)
    }



}