use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;


#[derive(Serialize, Deserialize, FromRow)]
pub struct ProgressionWOView {
    pub id: i32,
    pub wo_id: i32,
    pub wo_code: Option<String>,
    pub wo_name: Option<String>,
    pub region_id: Option<i32>,
    pub location: Option<String>,
    pub regional: Option<String>,
    pub witel: Option<String>,
    pub psa: Option<String>,
    pub survey_homepas: Option<String>,
    pub valid_homepas: Option<i32>,
    pub submit_vermit: Option<String>,
    pub valid_vermit: Option<i32>,
    pub description: Option<String>,
    pub status: Option<String>,
    pub attachment_path: Option<String>,
    pub date: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct CreateProgressionWO {
    pub wo_id: i32,
    pub date: Option<String>,
    pub description: Option<String>,
    pub attachment_path: Option<String>,
    pub status: Option<String>,
    pub survey_homepas: Option<String>,
    pub valid_homepas: Option<i32>,
    pub submit_vermit: Option<String>,
    pub valid_vermit: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct UpdateProgressionWO {
    pub id: i32,
    pub description: Option<String>,
    pub attachment_path: Option<String>,
    pub status: Option<String>,
    pub survey_homepas: Option<String>,
    pub valid_homepas: Option<i32>,
    pub submit_vermit: Option<String>,
    pub valid_vermit: Option<i32>,
}