use actix_web::{web, HttpResponse};
use sqlx::PgPool;

use crate::{CreateProject, GetProject, Project, UpdateProject};




// project handlers
pub async fn create_project(
    pool: web::Data<PgPool>,
    request: web::Json<CreateProject>
) -> HttpResponse {
    Project::create_project(request.into_inner(), &pool).await
}

pub async fn get_project(
    pool: web::Data<PgPool>,
    user_id: web::Path<GetProject>
) -> HttpResponse {
    let user_id = user_id.into_inner().user_id;
    Project::get_project(user_id, &pool).await
}

pub async fn update_project(
    pool: web::Data<PgPool>,
    request: web::Json<UpdateProject>
) -> HttpResponse {
    Project::update(request.into_inner(), &pool).await
}

pub async fn delete_project(
    pool: web::Data<PgPool>,
    id: web::Path<i32>
) -> HttpResponse {
    Project::delete(id.into_inner(), &pool).await
}