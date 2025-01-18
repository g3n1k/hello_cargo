use actix_web::{web, HttpResponse};
use sqlx::PgPool;

use crate::ProjectType;



// project handlers
pub async fn get_project_type(
    pool: web::Data<PgPool>
) -> HttpResponse {
    ProjectType::get(&pool).await
}