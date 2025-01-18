use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateProjectTracking {
    pub project_id: i32,
    pub stage: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetProjectTracking {
    pub project_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateProjectTracking {
    pub id: i32,
    pub project_id: i32,
    pub stage: String,
}