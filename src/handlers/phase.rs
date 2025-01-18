use actix_web::{web, HttpResponse};
use sqlx::PgPool;

use crate::uc_phase::Phase;



// project handlers
pub async fn get_phase(
    pool: web::Data<PgPool>
) -> HttpResponse {
    Phase::get(&pool).await
}