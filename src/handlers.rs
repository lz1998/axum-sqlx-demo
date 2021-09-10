use axum::{
    response::{Html, IntoResponse},
    Json,
};
use axum::http::StatusCode;
use crate::dto::{CreateUserReq, CreateUserResp};

pub async fn hello() -> Html<&'static str> {
    println!("hello");
    Html("<h1>Hello, World!</h1>")
}


pub async fn create_user(
    Json(req): Json<CreateUserReq>,
) -> impl IntoResponse {
    println!("{:#?}", req);
    let resp = CreateUserResp {
        ok: true
    };
    (StatusCode::OK, Json(resp))
}
