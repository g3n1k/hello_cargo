use actix_web::HttpResponse;
use sqlx::PgPool;

use crate::{repository, ApiResponse, ProjectType };

impl ProjectType {
    pub async fn get(pool: &PgPool) -> HttpResponse {
        match repository::get_project_type(pool).await
        {
            Ok(status) => HttpResponse::Ok().json(ApiResponse {
                status: true,
                message: "Project Type retrieved successfully".to_string(),
                data: Some(status),
            }),
            Err(e) => HttpResponse::InternalServerError().json(ApiResponse {
                status: false,
                message: format!("Failed to retrieve project type: {}", e),
                data: None::<Vec<ProjectType>>,
            }),
        }
    }
}