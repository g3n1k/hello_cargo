use actix_web::{web, HttpResponse};
use sqlx::PgPool;

use crate::{CreateProjectPO, ProjectPO};




// project handlers
pub async fn create_project_po(
    pool: web::Data<PgPool>,
    request: web::Json<CreateProjectPO>
) -> HttpResponse {
    ProjectPO::create(request.into_inner(), &pool).await
}

pub async fn get_project_po(
    pool: web::Data<PgPool>,
    user_id: web::Path<i32>
) -> HttpResponse {
    ProjectPO::get(user_id.into_inner(), &pool).await
}

pub async fn update_project_po(
    pool: web::Data<PgPool>,
    request: web::Json<ProjectPO>
) -> HttpResponse {
    ProjectPO::update(request.into_inner(), &pool).await
}

pub async fn delete_project_po(
    pool: web::Data<PgPool>,
    id: web::Path<i32>
) -> HttpResponse {
    ProjectPO::delete(id.into_inner(), &pool).await
}