use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DeleteProject {
    pub id: Option<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateProject {
    pub id: Option<i32>,
    pub po_id: Option<String>,
    pub wo_id: Option<String>,
    pub project_type_id: Option<i16>,
    pub regional: i32,
    pub witel: i32,
    pub psa: i32,
    pub pic: i32,
    pub unit: Option<String>,
    pub cable_volume: Option<i16>,
    pub pole: Option<i16>,
    pub port: Option<i16>,
    pub service_price: Option<i64>,
    pub material_price: Option<i64>,
    pub inc_material_price: Option<i64>,
    pub deploy_service_price: Option<i64>,
    pub description: Option<String>,
    pub attachment_path: Option<String>,
    pub service: Option<bool>,
    pub material: Option<bool>,
    pub status: Option<String>,
    pub phase: Option<String>,
    pub survey_md_doc: Option<String>,
    pub wo_doc: Option<String>,
    pub po_doc: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct GetProject {
    pub user_id: i32
}

#[derive(Serialize, Deserialize)]
pub struct CreateProject {
    pub po_id: Option<String>,
    pub wo_id: Option<String>,
    pub project_type_id: Option<i16>,
    pub regional: i32,
    pub witel: i32,
    pub psa: i32,
    pub unit: Option<String>,
    pub cable_volume: Option<i16>,
    pub pole: Option<i16>,
    pub port: Option<i16>,
    pub service_price: Option<i64>,
    pub material_price: Option<i64>,
    pub inc_material_price: Option<i64>,
    pub deploy_service_price: Option<i64>,
    pub description: Option<String>,
    pub attachment_path: Option<String>,
    pub service: Option<bool>,
    pub material: Option<bool>,
    pub status: Option<String>,
    pub pic: i32,
    pub phase: Option<String>,
    pub survey_md_doc: Option<String>,
    pub wo_doc: Option<String>,
    pub po_doc: Option<String>,
}