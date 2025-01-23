use actix_web::{web, HttpResponse};
use sqlx::PgPool;

use crate::Dashboard;





// project handlers
pub async fn get_summary_project(
    pool: web::Data<PgPool>,
) -> HttpResponse {
    Dashboard::get_summary_project(&pool).await
}

pub async fn get_summary_progress(
    pool: web::Data<PgPool>,
) -> HttpResponse {
    Dashboard::get_summary_progress(&pool).await
}

pub async fn get_daily_progress(
    pool: web::Data<PgPool>,
) -> HttpResponse {
    Dashboard::get_daily_progress(&pool).await
}

pub async fn get_progression_updates(
    pool: web::Data<PgPool>,
) -> HttpResponse {
    Dashboard::get_progression_updates(&pool).await
}

pub async fn get_regional_distribution(
    pool: web::Data<PgPool>,
) -> HttpResponse {
    Dashboard::get_regional_distribution(&pool).await
}