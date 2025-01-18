use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct ProjectType {
    pub r#type: String,
    pub id: i16,
}