use sqlx::{PgPool, Result};

use crate::ProjectType;

pub async fn get_project_type(pool: &PgPool) -> Result<Vec<ProjectType>>{
    let str = format!("select * from public.project_type");
    let query = sqlx::query_as::<_, ProjectType>(&str);
    query.fetch_all(pool).await
}