use sqlx::{PgPool, Result};

use crate::Status;

pub async fn get_status(pool: &PgPool) -> Result<Vec<Status>>{
    let str = format!("select * from public.status");
    let query = sqlx::query_as::<_, Status>(&str);
    query.fetch_all(pool).await
}