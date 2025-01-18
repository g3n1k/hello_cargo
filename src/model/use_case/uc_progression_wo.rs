use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;


#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct ProgressionWO {
    pub id: i32,
    pub wo_id: Option<i32>,
    pub date: Option<String>,
    pub description: Option<String>,
    pub attachment_path: Option<String>,
    pub status: Option<String>,
    pub survey_homepas: Option<String>,
    pub valid_homepas: Option<i32>,
    pub submit_vermit: Option<String>,
    pub valid_vermit: Option<i32>,
}