use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;


#[derive(Serialize, Deserialize, FromRow)]
pub struct ProjectWO {
    pub id: i32,
    pub wo_code: Option<String>,
    pub wo_name: Option<String>,
    pub project_type_id: Option<i16>,
    pub region_id: Option<i32>,
    pub user_id: Option<i32>,
    pub survey_md_doc: Option<String>,
    pub wo_doc: Option<String>,
    pub survey_md_price: Option<i32>,
}