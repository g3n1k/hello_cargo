use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;


#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct Reconciliation {
    pub id: i32,
    pub po_id: Option<i32>,
    pub survey_md_check: Option<bool>,
    pub material_check: Option<bool>,
    pub service_check: Option<bool>,
    pub last_update: Option<String>,
}