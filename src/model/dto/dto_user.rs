use serde::{Deserialize, Serialize};

use crate::{model::User, UserDetail};




#[derive(Debug, Serialize, Deserialize)]
pub struct GetUserRequest {
    pub user_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub user: User,
    pub area_ids: Vec<i32>,  // List of area IDs to assign
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUserRequest {
    pub user: User,
    pub area_ids: Vec<i32>,  // List of area IDs to assign
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: UserDetail,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32, // user id
    pub username: String,
    pub exp: i64, // expiration timestamp
}