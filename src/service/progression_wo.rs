use actix_web::HttpResponse;
use sqlx::PgPool;

use crate::{check_progress_wo_by_id, create_progress_wo, delete_progress_wo, get_user_credential, repo_progress_wo, repository, update_progress_wo, ApiResponse, CreateProgressionWO, ProgressionWO, ProgressionWOView };

use super::MessageResponse;

impl ProgressionWO {
    pub async fn get(pool: &PgPool, user_id: i32) -> HttpResponse {
        let user_info;
        if let Ok(crd) = get_user_credential(pool, user_id).await{
            // println!("credential: {crd:?}");
            user_info = crd;
        } else {
            return HttpResponse::InternalServerError().json(ApiResponse {
                status: false,
                message: format!("Can't get User Credential"),
                data: None::<Vec<ProgressionWOView>>,
            });
        };

        let progression = if user_info.role == Some(format!("admin")) {
            repository::repo_progress_wo::get_progression_wo(pool).await
        } else {
            repository::repo_progress_wo::get_progression_wo_by_region_ids(pool, user_info).await
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
                data: None::<Vec<ProgressionWO>>,
            }),
        }
    }

    pub async fn create(pool: &PgPool, req: CreateProgressionWO) -> HttpResponse {
        // Query the database for the user
        let progress_wo = repo_progress_wo::check_progress_wo(pool, Some(req.wo_id), req.date.clone()).await;
        if let Ok(_) = progress_wo {
            return HttpResponse::Conflict().json(MessageResponse {
                success: false,
                message: "Progress WO already exist".to_string(),
            });
        }

        let insert = create_progress_wo(pool, &req).await;            
        match insert {
            Ok(_id) => {
                HttpResponse::Ok().json(MessageResponse {
                    success: true,
                    message: "Insert data success".to_string(),
                })
            },
            Err(_e) => {
                HttpResponse::InternalServerError().json(MessageResponse {
                    success: false,
                    message: "Insert data failed".to_string(),
                })
            },
        }
    }

    pub async fn update(pool: &PgPool, req: ProgressionWO) -> HttpResponse {
        // Query the database for the user
        let progress_wo = repo_progress_wo::check_progress_wo_by_id(pool, req.id).await;
        if let Err(e) = progress_wo {
            println!("error: {e}");
            return HttpResponse::InternalServerError().json(MessageResponse {
                success: false,
                message: "Progress WO doesn't exist".to_string(),
            });
        }

        let insert = update_progress_wo(pool, &req).await;            
        match insert {
            Ok(_id) => {
                HttpResponse::Ok().json(MessageResponse {
                    success: true,
                    message: "Update data success".to_string(),
                })
            },
            Err(_e) => {
                HttpResponse::InternalServerError().json(MessageResponse {
                    success: false,
                    message: "Update data failed".to_string(),
                })
            },
        }
    }

    pub async fn delete(pool: &PgPool, id: i32) -> HttpResponse {
        let project_exists = check_progress_wo_by_id(pool, id).await;
        match project_exists {
            Ok(_) => { 
                let delete_result = delete_progress_wo(pool, id).await;
                match delete_result {
                    Ok(_) => HttpResponse::Ok().json(MessageResponse {
                        success: true,
                        message: "Delete data success".to_string(),
                    }),
                    Err(e) => {
                        println!("Delete error: {:?}", e); // Logging error
                        HttpResponse::InternalServerError().json(MessageResponse {
                            success: false,
                            message: format!("Delete data failed: {}", e),
                        })
                    }
                }
            },
            Err(_e) => {
                HttpResponse::NotFound().json(MessageResponse {
                    success: false,
                    message: format!("Progress WO with ID {} not found", id),
                })
            }
        }
    }
}