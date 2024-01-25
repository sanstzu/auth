use sqlx::{Pool, Postgres};

use crate::models::User;

impl User {
    pub async fn query_by_id(pool: &Pool<Postgres>, id: &str) -> Result<Option<User>, sqlx::Error> {
        let query = "SELECT * 
    FROM users WHERE id = $1";
        let res: Option<User> = sqlx::query_as::<_, User>(query)
            .bind(id)
            .fetch_optional(pool)
            .await?;

        Ok(res)
    }

    pub async fn query_by_username(
        pool: &Pool<Postgres>,
        id: &str,
    ) -> Result<Option<User>, sqlx::Error> {
        let query = "SELECT * 
    FROM users WHERE username = $1";
        let res: Option<User> = sqlx::query_as::<_, User>(query)
            .bind(id)
            .fetch_optional(pool)
            .await?;

        Ok(res)
    }

    pub async fn add_user(pool: &Pool<Postgres>, user: &User) -> Result<(), sqlx::Error> {
        let query_raw = "INSERT INTO users(id, username, password) 
    VALUES ($1, $2, $3)";

        sqlx::query(query_raw)
            .bind(user.get_id())
            .bind(user.get_username())
            .bind(user.get_password())
            .execute(pool)
            .await?;

        Ok(())
    }
}
