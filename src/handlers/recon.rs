use actix_web::{web, HttpResponse};
use sqlx::PgPool;

use crate::{CreateRecon, Reconciliation};




// project handlers
pub async fn get_recon_po(
    pool: web::Data<PgPool>,
    user_id: web::Path<i32>,
) -> HttpResponse {
    Reconciliation::get_po(&pool, user_id.into_inner()).await
}

pub async fn get_recon(
    pool: web::Data<PgPool>,
    user_id: web::Path<i32>
) -> HttpResponse {
    Reconciliation::get(&pool, user_id.into_inner()).await
}

pub async fn create_recon(
    pool: web::Data<PgPool>,
    req: web::Json<CreateRecon>
) -> HttpResponse {
    Reconciliation::create(&pool, req.0).await
}

pub async fn update_recon(
    pool: web::Data<PgPool>,
    req: web::Json<Reconciliation>
) -> HttpResponse {
    Reconciliation::update(&pool, req.0).await
}

pub async fn delete_recon(
    pool: web::Data<PgPool>,
    id: web::Path<i32>
) -> HttpResponse {
    Reconciliation::delete(&pool, id.into_inner()).await
}