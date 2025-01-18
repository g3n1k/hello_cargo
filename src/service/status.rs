use actix_web::HttpResponse;
use sqlx::PgPool;

use crate::{repository, uc_status::Status, ApiResponse };

impl Status {
    pub async fn get(pool: &PgPool) -> HttpResponse {
        match repository::get_status(pool).await
        {
            Ok(status) => HttpResponse::Ok().json(ApiResponse {
                status: true,
                message: "Status retrieved successfully".to_string(),
                data: Some(status),
            }),
            Err(e) => HttpResponse::InternalServerError().json(ApiResponse {
                status: false,
                message: format!("Failed to retrieve status: {}", e),
                data: None::<Vec<Status>>,
            }),
        }
    }
}