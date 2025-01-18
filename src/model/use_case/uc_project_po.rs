use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;


#[derive(Serialize, Deserialize, FromRow)]
pub struct ProjectPO {
    pub id: i32,
    pub po_code: Option<String>,
    pub po_name: Option<String>,
    pub wo_id: Option<i32>,
    pub user_id: Option<i32>,
    pub cable: Option<i32>,
    pub pole: Option<i32>,
    pub port: Option<i32>,
    pub material_price: Option<i32>,
    pub service_price: Option<i32>,
    pub po_doc: Option<String>,
}