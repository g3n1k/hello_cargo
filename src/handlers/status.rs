use actix_web::{web, HttpResponse};
use sqlx::PgPool;

use crate::uc_status::Status;



// project handlers
pub async fn get_status(
    pool: web::Data<PgPool>
) -> HttpResponse {
    Status::get(&pool).await
}