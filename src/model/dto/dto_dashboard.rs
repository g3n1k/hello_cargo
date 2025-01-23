use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;


#[derive(Serialize, Deserialize, FromRow)]
pub struct SummaryProjectView {
    pub name: Option<String>,
    pub value1: Option<String>,
    pub value2: Option<String>,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct SummaryProgressionView {
    pub name: Option<String>,
    pub value: Option<i64>,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct DailyProgressionView {
    pub date: Option<String>,
    pub wo: Option<i64>,
    pub po: Option<i64>,
    pub total: Option<i64>,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct ProgressionUpdatesView {
    pub date: Option<String>,
    pub project: Option<String>,
    pub update: Option<String>,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct RegionalDistributionView {
    pub regional: Option<String>,
    pub value: Option<i64>,
}