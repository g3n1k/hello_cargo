use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;



#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub full_name: String,
    pub email: String,
    pub phone_number: String,
    pub role_id: Option<i32>,
    pub area_assigned_id: Option<i32>,
    pub team_id: Option<i32>,
    pub employee_id: String,
    pub status: Option<String>,
    pub date_of_birth: Option<String>,
    pub join_date: Option<String>,
    pub last_login: Option<String>,
    pub gender: Option<String>,
    pub address: Option<String>,
    pub profile_picture: Option<String>,
    pub notes: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>, // Add created_at field
    pub updated_at: Option<chrono::NaiveDateTime>, //
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AreaAssigned {
    pub id: i32,
    pub area_name: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct PIC {
    pub id: i32,
    pub pic: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserAreaAssigned {
    pub user_id: i32,
    pub area_assigned_id: i32,
}

#[derive(Debug, Serialize)]
pub struct UserCredential {
    pub user_id: i32,
    pub role: Option<String>,
    pub region_level: Option<String>,
    pub region_id: Option<Vec<i32>>,
}

// Add this struct for joined user data
#[derive(Debug, Serialize, FromRow)]
pub struct UserDetail {
    // User fields
    pub id: i32,
    pub username: String,
    pub full_name: String,
    pub email: String,
    pub phone_number: String,
    pub employee_id: String,
    pub status: Option<String>,
    pub date_of_birth: Option<String>,
    pub join_date: Option<String>,
    pub last_login: Option<String>,
    pub gender: Option<String>,
    pub address: Option<String>,
    pub profile_picture: Option<String>,
    pub notes: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
    
    // Joined fields
    pub role: Option<String>,
    pub team_name: Option<String>,
    pub assigned_areas: Option<Vec<String>>, // List of assigned area names
}