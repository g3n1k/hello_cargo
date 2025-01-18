use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;


#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct ProgressionPO {
    pub id: i32,
    pub po_id: Option<i32>,
    pub date: Option<String>,
    pub description: Option<String>,
    pub attachment_path: Option<String>,
    pub status: Option<String>,
    pub phase: Option<String>,   
}