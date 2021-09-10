use axum::{
    response::{Html, IntoResponse},
    Json,
    http::StatusCode,
    extract::Extension,
};
use tower::BoxError;
use std::convert::Infallible;
use sqlx::{Pool, MySql};
use crate::dto;
use crate::dao;

pub async fn hello() -> Html<&'static str> {
    println!("hello");
    Html("<h1>Hello, World!</h1>")
}

pub async fn create_user(
    Json(req): Json<dto::CreateUserReq>,
    Extension(pool): Extension<Pool<MySql>>,
) -> impl IntoResponse {
    println!("{:#?}", req);
    match dao::create_user(&pool, &req.username, &req.password).await {
        Ok(_) => {
            (StatusCode::OK, Json(dto::CreateUserResp {
                ok: true
            }))
        }
        Err(_) => {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(dto::CreateUserResp {
                ok: false
            }))
        }
    }
}

pub async fn delete_user(
    Json(req): Json<dto::DeleteUserReq>,
    Extension(pool): Extension<Pool<MySql>>,
) -> impl IntoResponse {
    println!("{:#?}", req);
    match dao::delete_user(&pool, req.id).await {
        Ok(_) => {
            (StatusCode::OK, Json(dto::DeleteUserResp {
                ok: true
            }))
        }
        Err(_) => {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(dto::DeleteUserResp {
                ok: false
            }))
        }
    }
}