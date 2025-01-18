use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::ProgressType;



#[derive(Serialize, Deserialize, FromRow)]
pub struct ProgressionForm {
    pub id: i32,
    pub project_id: i32,
    pub project_name: String,
    pub date: Option<String>,
    pub team: Option<String>,
    pub pic: Option<String>,
    pub regional: Option<String>,
    pub witel: Option<String>,
    pub psa: Option<String>,
    pub phone_number: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
    pub phase: Option<String>,
    pub progress_type: Option<ProgressType>,
    pub attachment_path: Option<String>,
}