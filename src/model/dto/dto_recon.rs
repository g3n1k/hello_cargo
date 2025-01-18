use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Serialize, Deserialize)]
pub struct CreateRecon {
    pub po_id: Option<i32>,
    pub survey_md_check: Option<bool>,
    pub material_check: Option<bool>,
    pub service_check: Option<bool>,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct ReconPo {
    pub po_id: Option<i32>,
    pub wo_id: Option<i32>,
    pub po_name: Option<String>,
    pub wo_name: Option<String>,
    pub project_name: Option<String>,
    pub survey_md_price: Option<i32>,
    pub material_price: Option<i32>,
    pub service_price: Option<i32>,
    pub survey_md_check: Option<bool>,
    pub material_check: Option<bool>,
    pub service_check: Option<bool>,
    pub status: Option<String>,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct ReconView {
    pub id: i32,
    pub po_id: Option<i32>,
    pub po_name: Option<String>,
    pub wo_name: Option<String>,
    pub project_type: Option<String>,
    pub status: Option<String>,
    pub survey_md_price: Option<i32>,
    pub survey_md_check: Option<bool>,
    pub material_price: Option<i32>,
    pub material_check: Option<bool>,
    pub service_price: Option<i32>,
    pub service_check: Option<bool>,
    pub last_update: Option<String>,
}