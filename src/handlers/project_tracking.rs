use actix_web::{web, HttpResponse};
use sqlx::PgPool;

use crate::{CreateProjectTracking, GetProjectTracking, ProjectTracking, UpdateProjectTracking};

// project tracking handlers
pub async fn create_project_tracking(
    pool: web::Data<PgPool>,
    request: web::Json<CreateProjectTracking>
) -> HttpResponse {
    ProjectTracking::create(request.into_inner(), &pool).await
}

pub async fn get_project_tracking(
    pool: web::Data<PgPool>,
    user_id: web::Path<i32>,
    request: web::Query<GetProjectTracking>,
) -> HttpResponse {
    ProjectTracking::get(&pool, user_id.into_inner(), request.into_inner()).await
}

pub async fn update_project_tracking(
    pool: web::Data<PgPool>,
    request: web::Json<UpdateProjectTracking>
) -> HttpResponse {
    ProjectTracking::update(request.into_inner(), &pool).await
}

pub async fn delete_project_tracking(
    pool: web::Data<PgPool>,
    id: web::Path<i32>
) -> HttpResponse {
    ProjectTracking::delete(id.into_inner(), &pool).await
}