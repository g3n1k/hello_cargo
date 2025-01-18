use sqlx::{PgPool, Result};

use crate::uc_phase::Phase;


pub async fn get_phase(pool: &PgPool) -> Result<Vec<Phase>>{
    let str = format!("select * from public.phase");
    let query = sqlx::query_as::<_, Phase>(&str);
    query.fetch_all(pool).await
}