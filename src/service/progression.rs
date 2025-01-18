use actix_web::HttpResponse;
use sqlx::PgPool;

use crate::{get_user_credential, repository, uc_progression::Progression, ApiResponse, ProgressionForm };

impl Progression {
    pub async fn get(pool: &PgPool, user_id: i32) -> HttpResponse {
        let user_info;
        if let Ok(crd) = get_user_credential(pool, user_id).await{
            // println!("credential: {crd:?}");
            user_info = crd;
        } else {
            return HttpResponse::InternalServerError().json(ApiResponse {
                status: false,
                message: format!("Can't get User Credential"),
                data: None::<Vec<ProgressionForm>>,
            });
        };

        let progression = if user_info.role == Some(format!("admin")) {
            repository::repo_progress::get_progression(pool).await
        } else {
            repository::repo_progress::get_progression_by_region_ids(pool, user_info).await
        };

        match progression {
            Ok(progress) => HttpResponse::Ok().json(ApiResponse {
                status: true,
                message: "Progress retrieved successfully".to_string(),
                data: Some(progress),
            }),
            Err(e) => HttpResponse::InternalServerError().json(ApiResponse {
                status: false,
                message: format!("Failed to retrieve progress: {}", e),
                data: None::<Vec<Progression>>,
            }),
        }
    }
    
    pub async fn get_by_id(pool: &PgPool, id: i32) -> HttpResponse {
        match repository::get_progression_by_project_id(pool, id).await
        {
            Ok(progress) => HttpResponse::Ok().json(ApiResponse {
                status: true,
                message: "Progress retrieved successfully".to_string(),
                data: Some(progress),
            }),
            Err(e) => HttpResponse::InternalServerError().json(ApiResponse {
                status: false,
                message: format!("Failed to retrieve progress: {}", e),
                data: None::<Vec<Progression>>,
            }),
        }
    }
}