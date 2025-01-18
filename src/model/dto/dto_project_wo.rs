use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Serialize, Deserialize)]
pub struct CreateProjectWO {
    pub wo_name: Option<String>,
    pub project_type_id: Option<i16>,
    pub pic: Option<i32>,
    pub region_id: Option<i32>,
    pub survey_md_doc: Option<String>,
    pub wo_doc: Option<String>,
    pub survey_md_price: Option<i32>,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct ProjectWOView {
    pub id: Option<i32>,
    pub wo_code: Option<String>,
    pub wo_name: Option<String>,
    pub project_type_id: Option<i16>,
    pub project_type: Option<String>,
    pub region_id: Option<i32>,
    pub regional_id: Option<i32>,
    pub witel_id: Option<i32>,
    pub psa_id: Option<i32>,
    pub user_id: Option<i32>,
    pub team_id: Option<i32>,
    pub location: Option<String>,
    pub regional: Option<String>,
    pub witel: Option<String>,
    pub psa: Option<String>,
    pub pic: Option<String>,
    pub team: Option<String>,
    pub wo_doc: Option<String>,
    pub survey_md_doc: Option<String>,
    pub survey_md_price: Option<i32>,
}