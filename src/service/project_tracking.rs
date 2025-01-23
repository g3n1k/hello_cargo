use actix_web::HttpResponse;
use sqlx::PgPool;

use crate::{MessageResponse, repo_project_tracking, repository::{check_project_tracking_by_id, create_project_tracking, delete_project_tracking, update_project_tracking}, ApiResponse, CreateProjectTracking, GetProjectTracking, ProjectTracking, UpdateProjectTracking};

impl ProjectTracking {
    pub async fn create(req: CreateProjectTracking, pool: &PgPool) -> HttpResponse {
        // Query the database for the user
        let project_tracking = repo_project_tracking::check_project_tracking(pool, &req).await;
        if let Ok(_) = project_tracking {
            return HttpResponse::Conflict().json(MessageResponse {
                success: false,
                message: "Project Tracking already exist".to_string(),
            });
        }

        let insert = create_project_tracking(pool, &req).await;            
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

    pub async fn get(pool: &PgPool, _user_id: i32, req: GetProjectTracking) -> HttpResponse {
        let project_id = req.project_id;
        let project_tracking = repo_project_tracking::get_project_tracking_by_project_id(pool, project_id).await;
        match project_tracking {
            Ok(progress) => HttpResponse::Ok().json(ApiResponse {
                status: true,
                message: "Progress retrieved successfully".to_string(),
                data: Some(progress),
            }),
            Err(e) => HttpResponse::InternalServerError().json(ApiResponse {
                status: false,
                message: format!("Failed to retrieve progress: {}", e),
                data: None::<Vec<ProjectTracking>>,
            }),
        }
    }
    
    pub async fn update(req: UpdateProjectTracking, pool: &PgPool) -> HttpResponse {
        // Query the database for the user
        let project_tracking = repo_project_tracking::check_project_tracking_by_id(pool, req.id).await;
        if let Err(_) = project_tracking {
            return HttpResponse::InternalServerError().json(MessageResponse {
                success: false,
                message: "Project Tracking not exist".to_string(),
            });
        }

        let update = update_project_tracking(pool, &req).await;            
        match update {
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
    
    pub async fn delete(id: i32, pool: &PgPool) -> HttpResponse {
        let project_exists = check_project_tracking_by_id(pool, id).await;
        match project_exists {
            Ok(_) => { 
                let delete_result = delete_project_tracking(pool, id).await;
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
                    message: format!("Project with ID {} not found", id),
                })
            }
        }
    }
}