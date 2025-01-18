use actix_web::{web, HttpResponse};
use sqlx::PgPool;

use crate::{CreateProgressionPO, ProgressionPO};



// project handlers
pub async fn get_progression_po(
    pool: web::Data<PgPool>,
    user_id: web::Path<i32>
) -> HttpResponse {
    ProgressionPO::get(&pool, user_id.into_inner()).await
}

// pub async fn get_progression_by_id(
//     pool: web::Data<PgPool>,
//     id: web::Path<i32>,
// ) -> HttpResponse {
//     Progression::get_by_id(&pool, id.into_inner()).await
// }

pub async fn create_progression_po(
    pool: web::Data<PgPool>,
    request: web::Json<CreateProgressionPO>
) -> HttpResponse {
    ProgressionPO::create(&pool, request.into_inner()).await
}

pub async fn update_progression_po(
    pool: web::Data<PgPool>,
    request: web::Json<ProgressionPO>
) -> HttpResponse {
    ProgressionPO::update(&pool, request.into_inner()).await
}

pub async fn delete_progression_po(
    pool: web::Data<PgPool>,
    user_id: web::Path<i32>
) -> HttpResponse {
    ProgressionPO::delete(&pool, user_id.into_inner()).await
}