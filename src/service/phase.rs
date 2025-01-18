use actix_web::HttpResponse;
use sqlx::PgPool;

use crate::{repository, uc_phase::Phase, ApiResponse };

impl Phase {
    pub async fn get(pool: &PgPool) -> HttpResponse {
        match repository::get_phase(pool).await
        {
            Ok(phase) => HttpResponse::Ok().json(ApiResponse {
                status: true,
                message: "Phase retrieved successfully".to_string(),
                data: Some(phase),
            }),
            Err(e) => HttpResponse::InternalServerError().json(ApiResponse {
                status: false,
                message: format!("Failed to retrieve phase: {}", e),
                data: None::<Vec<Phase>>,
            }),
        }
    }
}