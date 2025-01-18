use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct ProjectTracking {
    pub id: i32,
    pub project_id: i32,
    pub stage: String,
}

