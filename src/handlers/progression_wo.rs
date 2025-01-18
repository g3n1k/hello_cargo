use actix_web::{web, HttpResponse};
use sqlx::PgPool;

use crate::{CreateProgressionWO, ProgressionWO};



// project handlers
pub async fn get_progression_wo(
    pool: web::Data<PgPool>,
    user_id: web::Path<i32>
) -> HttpResponse {
    ProgressionWO::get(&pool, user_id.into_inner()).await
}

// pub async fn get_progression_by_id(
//     pool: web::Data<PgPool>,
//     id: web::Path<i32>,
// ) -> HttpResponse {
//     Progression::get_by_id(&pool, id.into_inner()).await
// }

pub async fn create_progression_wo(
    pool: web::Data<PgPool>,
    request: web::Json<CreateProgressionWO>
) -> HttpResponse {
    ProgressionWO::create(&pool, request.into_inner()).await
}

pub async fn update_progression_wo(
    pool: web::Data<PgPool>,
    request: web::Json<ProgressionWO>
) -> HttpResponse {
    ProgressionWO::update(&pool, request.into_inner()).await
}

pub async fn delete_progression_wo(
    pool: web::Data<PgPool>,
    user_id: web::Path<i32>
) -> HttpResponse {
    ProgressionWO::delete(&pool, user_id.into_inner()).await
}