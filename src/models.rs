use sqlx::{Error, MySql, Pool, FromRow};
use sqlx::mysql::MySqlQueryResult;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, FromRow, Clone)]
pub struct User {
    pub id: u64,
    pub username: String,
    pub password: String,
}

pub async fn create_user(pool: &Pool<MySql>, username: &str, password: &str) -> Result<MySqlQueryResult, Error> {
    sqlx::query(
        r#"
            INSERT INTO user (`username`, `password`)
            VALUES(?, ?)"#,
    )
        .bind(username)
        .bind(password)
        .execute(pool).await
}

pub async fn delete_user(pool: &Pool<MySql>, id: u64) -> Result<MySqlQueryResult, Error> {
    sqlx::query(
        r#"
            DELETE FROM user
            WHERE id = ?
            "#,
    )
        .bind(id)
        .execute(pool).await
}


pub async fn get_users(pool: &Pool<MySql>, ids: Vec<u64>) -> Result<Vec<User>, Error> {
    sqlx::query_as("SELECT * FROM user WHERE id in (?)").bind(ids).fetch_all(pool).await
}
