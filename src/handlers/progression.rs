use actix_web::{web, HttpResponse};
use sqlx::PgPool;

use crate::uc_progression::Progression;



// project handlers
pub async fn get_progression(
    pool: web::Data<PgPool>,
    user_id: web::Path<i32>
) -> HttpResponse {
    Progression::get(&pool, user_id.into_inner()).await
}

pub async fn get_progression_by_id(
    pool: web::Data<PgPool>,
    id: web::Path<i32>,
) -> HttpResponse {
    Progression::get_by_id(&pool, id.into_inner()).await
}