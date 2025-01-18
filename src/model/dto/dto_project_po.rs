use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Serialize, Deserialize)]
pub struct CreateProjectPO {
    pub po_name: Option<String>,
    pub wo_id: Option<i32>,
    pub pic: Option<i32>,
    pub po_doc: Option<String>,
    pub cable: Option<i32>,
    pub pole: Option<i32>,
    pub port: Option<i32>,
    pub material_price: Option<i32>,
    pub service_price: Option<i32>,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct ProjectPOView {
    pub id: Option<i32>,
    pub po_code: Option<String>,
    pub po_name: Option<String>,
    pub wo_id: Option<i32>,
    pub wo_name: Option<String>,
    pub project_type_id: Option<i16>,
    pub project_type: Option<String>,
    pub region_id: Option<i32>,
    pub location: Option<String>,
    pub cable: Option<i32>,
    pub pole: Option<i32>,
    pub port: Option<i32>,
    pub user_id: Option<i32>,
    pub pic: Option<String>,
    pub team_id: Option<i32>,
    pub team: Option<String>,
    pub po_doc: Option<String>,
    pub material_price: Option<i32>,
    pub service_price: Option<i32>,
}