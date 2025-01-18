use actix_web::{web, HttpResponse};
use sqlx::PgPool;

use crate::{CreateProjectWO, ProjectWO};




// project handlers
pub async fn create_project_wo(
    pool: web::Data<PgPool>,
    request: web::Json<CreateProjectWO>
) -> HttpResponse {
    ProjectWO::create(request.into_inner(), &pool).await
}

pub async fn get_project_wo(
    pool: web::Data<PgPool>,
    user_id: web::Path<i32>
) -> HttpResponse {
    ProjectWO::get(user_id.into_inner(), &pool).await
}

pub async fn update_project_wo(
    pool: web::Data<PgPool>,
    request: web::Json<ProjectWO>
) -> HttpResponse {
    ProjectWO::update(request.into_inner(), &pool).await
}

pub async fn delete_project_wo(
    pool: web::Data<PgPool>,
    id: web::Path<i32>
) -> HttpResponse {
    ProjectWO::delete(id.into_inner(), &pool).await
}