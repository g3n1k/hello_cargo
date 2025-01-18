use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, sqlx::Type)]
#[sqlx(rename_all = "lowercase", type_name = "progress_type")]
pub enum ProgressType {
    PO,
    WO,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct Progression {
    pub id: i32,
    pub project_id: i32,
    pub date: Option<String>,
    pub description: Option<String>,
    pub attachment_path: Option<String>,
    pub status: Option<String>,
    pub phase: Option<String>,
    pub progress_type: Option<ProgressType>,    
}