use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Regional {
    pub regional: Option<String>,
    pub id: i32,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct Witel {
    pub witel: Option<String>,
    pub id: i32,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct PSA {
    pub psa: Option<String>,
    pub id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Region {
    pub id: i32,
}