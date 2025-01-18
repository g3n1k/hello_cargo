use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;


#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct Billing {
    pub id: i32,
    pub recon_id: Option<i32>,
    pub status: Option<String>,
    pub create_time: Option<String>,
    pub update_time: Option<String>,
}