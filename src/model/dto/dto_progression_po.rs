use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;


#[derive(Serialize, Deserialize, FromRow)]
pub struct CreateProgressionPO {
    pub po_id: i32,
    pub date: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
    pub phase: Option<String>,
    pub attachment_path: Option<String>,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct ProgressionPOView {
    pub id: i32,
    pub po_id: Option<i32>,
    pub po_code: Option<String>,
    pub po_name: Option<String>,
    pub date: Option<String>,
    pub region_id: Option<i32>,
    pub location: Option<String>,
    pub regional: Option<String>,
    pub witel: Option<String>,
    pub psa: Option<String>,
    pub team: Option<String>,
    pub pic: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
    pub phase: Option<String>,
    pub attachment_path: Option<String>,
}