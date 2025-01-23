use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;


#[derive(Serialize, Deserialize, FromRow)]
pub struct ProjectHighlightView {
    pub total_project: Option<i64>,
    pub cable: Option<i64>,
    pub survey_md_price: Option<i64>,
    pub material_price: Option<i64>,
    pub service_price: Option<i64>,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct ProgressionPOHighlightView {
    pub total_progress: Option<i64>,
    pub cable: Option<i64>,
    pub pole: Option<i64>,
    pub port: Option<i64>,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct ProgressionWOHighlightView {
    pub total_progress: Option<i64>,
    pub survey_homepas: Option<i64>,
    pub valid_vermit: Option<i64>,
    pub submit_vermit: Option<i64>,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct ReconHighlightView {
    pub total_recon: Option<i64>,
    pub survey_md_price: Option<i64>,
    pub material_price: Option<i64>,
    pub total_price: Option<i64>,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct BillingHighlightView {
    pub total_billing: Option<i64>,
    pub recon: Option<i64>,
    pub in_billing: Option<i64>,
    pub paid: Option<i64>,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct UserManagementHighlightView {
    pub total_user: Option<i64>,
    pub ho: Option<i64>,
    pub pm: Option<i64>,
    pub tl: Option<i64>,
}