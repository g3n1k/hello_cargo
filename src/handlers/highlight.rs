use actix_web::{web, HttpResponse};
use sqlx::PgPool;

use crate::Highlight;





// project handlers
pub async fn get_project_highlight(
    pool: web::Data<PgPool>,
    user_id: web::Path<i32>,
) -> HttpResponse {
    Highlight::get_project_highlight(&pool, user_id.into_inner()).await
}

pub async fn get_progression_po_highlight(
    pool: web::Data<PgPool>,
    user_id: web::Path<i32>
) -> HttpResponse {
    Highlight::get_progression_po_highlight(&pool, user_id.into_inner()).await
}

pub async fn get_progression_wo_highlight(
    pool: web::Data<PgPool>,
    user_id: web::Path<i32>
) -> HttpResponse {
    Highlight::get_progression_wo_highlight(&pool, user_id.into_inner()).await
}

pub async fn get_recon_highlight(
    pool: web::Data<PgPool>,
    user_id: web::Path<i32>
) -> HttpResponse {
    Highlight::get_recon_highlight(&pool, user_id.into_inner()).await
}

pub async fn get_billing_highlight(
    pool: web::Data<PgPool>,
    user_id: web::Path<i32>
) -> HttpResponse {
    Highlight::get_billing_highlight(&pool, user_id.into_inner()).await
}

pub async fn get_user_management_highlight(
    pool: web::Data<PgPool>,
    user_id: web::Path<i32>
) -> HttpResponse {
    Highlight::get_user_management_highlight(&pool, user_id.into_inner()).await
}