use serde::{Deserialize, Serialize};
use crate::models;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateUserReq {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateUserResp {
    pub ok: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeleteUserReq {
    pub id: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeleteUserResp {
    pub ok: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetUsersReq {
    pub ids: Vec<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetUsersResp {
    pub users: Vec<models::User>,
}
