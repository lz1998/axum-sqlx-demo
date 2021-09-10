use sqlx::{Error, MySql, Pool};
use sqlx::mysql::MySqlQueryResult;

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