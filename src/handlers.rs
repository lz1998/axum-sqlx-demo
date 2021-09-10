use axum::{
    response::{Html, IntoResponse},
    Json,
    http::StatusCode,
    extract::Extension,
};
use tower::BoxError;
use std::convert::Infallible;
use sqlx::{Pool, MySql};
use crate::dto::{CreateUserReq, CreateUserResp};

pub async fn hello() -> Html<&'static str> {
    println!("hello");
    Html("<h1>Hello, World!</h1>")
}


pub async fn create_user(
    Json(req): Json<CreateUserReq>,
    Extension(pool): Extension<Pool<MySql>>,
) -> impl IntoResponse {
    println!("{:#?}", req);
    match sqlx::query(
        r#"
            INSERT INTO user (`username`, `password`)
            VALUES(?, ?)"#,
    )
        .bind(&req.username)
        .bind(&req.password)
        .execute(&pool).await {
        Ok(_) => {
            (StatusCode::OK, Json(CreateUserResp {
                ok: true
            }))
        }
        Err(_) => {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(CreateUserResp {
                ok: false
            }))
        }
    }
}
