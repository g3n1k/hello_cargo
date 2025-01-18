use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;


#[derive(Serialize, Deserialize, FromRow)]
pub struct CreateBilling {
    pub recon_id: Option<i32>,
    pub status: Option<String>,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct BillingView {
    pub id: i32,
    pub po_id: Option<i32>,
    pub po_code: Option<String>,
    pub po_name: Option<String>,
    pub wo_id: Option<i32>,
    pub wo_code: Option<String>,
    pub wo_name: Option<String>,
    pub project_name: Option<String>,
    pub project_type_id: Option<i16>,
    pub project_type: Option<String>,
    pub region_id: Option<i32>,
    pub location: Option<String>,
    pub regional: Option<String>,
    pub witel: Option<String>,
    pub psa: Option<String>,
    pub team_id: Option<i32>,
    pub team_name: Option<String>,
    pub user_id: Option<i32>,
    pub pic: Option<String>,
    pub survey_homepas: Option<String>,
    pub valid_homepas: Option<i64>,
    pub submit_vermit: Option<String>,
    pub valid_vermit: Option<i64>,
    pub cable: Option<i32>,
    pub pole: Option<i32>,
    pub port: Option<i32>,
    pub survey_md_price: Option<i32>,
    pub survey_md_check: Option<bool>,
    pub material_price: Option<i32>,
    pub material_check: Option<bool>,
    pub service_price: Option<i32>,
    pub service_check: Option<bool>,
    pub total_price: Option<i32>,
    pub description: Option<String>,
    pub phase: Option<String>,
    pub po_status: Option<String>,
    pub status: Option<String>,
    pub last_update: Option<String>,
}