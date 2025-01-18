use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GetRegional {
    pub user_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct GetWitel {
    pub user_id: i32,
    pub regional: i32,
}

#[derive(Serialize, Deserialize)]
pub struct GetPSA {
    pub user_id: i32,
    pub regional: i32,
    pub witel: i32,
}