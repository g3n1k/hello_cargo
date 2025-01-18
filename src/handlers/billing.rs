use actix_web::{web, HttpResponse};
use sqlx::PgPool;

use crate::Billing;




// project handlers
pub async fn get_billing(
    pool: web::Data<PgPool>,
    user_id: web::Path<i32>
) -> HttpResponse {
    Billing::get(&pool, user_id.into_inner()).await
}

pub async fn update_billing(
    pool: web::Data<PgPool>,
    req: web::Json<Billing>
) -> HttpResponse {
    Billing::update(&pool, req.0).await
}