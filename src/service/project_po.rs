use actix_web::HttpResponse;
use sqlx::PgPool;

use crate::{MessageResponse, check_project_po, check_project_po_by_id, repo_project_po::{delete_project_po, update_project_po}, get_user_credential, insert_project_po, repository, ApiResponse, CreateProjectPO, ProjectPO, ProjectPOView};



impl ProjectPO {
    pub async fn create(req: CreateProjectPO, pool: &PgPool) -> HttpResponse {
        // Query the database for the user
        let project = check_project_po(pool, &req).await;
        if let Ok(_) = project {
            return HttpResponse::Conflict().json(MessageResponse {
                success: false,
                message: "PO already exist".to_string(),
            });
        }

        let insert = insert_project_po(pool, &req).await;            
        match insert {
            Ok(_project) => {                
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
    
    pub async fn get(user_id: i32, pool: &PgPool) -> HttpResponse {
        let role;
        let region_id;
        if let Ok(crd) = get_user_credential(pool, user_id).await{
            // println!("credential: {crd:?}");
            role = crd.role;
            region_id = crd.region_id;
        } else {
            return HttpResponse::InternalServerError().json(ApiResponse {
                status: false,
                message: format!("Can't get User Credential"),
                data: None::<Vec<ProjectPO>>,
            });
        };

        let projects = if role == Some(format!("admin")) {
            repository::repo_project_po::get_project_po_all(pool).await
        } else {
            repository::repo_project_po::get_project_po_by_regions(pool, region_id).await
        };

        match projects {
            Ok(project) => {
                HttpResponse::Ok().json(ApiResponse {
                    status: true,
                    message: format!("Get data success"),
                    data: Some(project),
                })
            }, 
            Err(err) => {
                HttpResponse::Ok().json(ApiResponse{ 
                    status: false, 
                    message: format!("Error {}", err), 
                    data: None::<Vec<ProjectPOView>>,
                })
            }
        }
    }

    pub async fn update(req: ProjectPO, pool: &PgPool) -> HttpResponse {
        let project_exists: Result<ProjectPO, sqlx::Error> = check_project_po_by_id(pool, req.id).await;    
        if let Err(_) = project_exists {
            return HttpResponse::NotFound().json(MessageResponse {
                success: false,
                message: format!("Project with ID {} not found", req.id),
            });
        }
    
        let update = update_project_po(pool, &req).await;    
        match update {
            Ok(_project_id) => {                
                HttpResponse::Ok().json(MessageResponse {
                    success: true,
                    message: "Update data success".to_string(),
                })
            }
            Err(e) => {
                println!("Update error: {:?}", e); // Logging error
                HttpResponse::InternalServerError().json(MessageResponse {
                    success: false,
                    message: "Update data failed".to_string(),
                })
            }
        }
    }
    
    pub async fn delete(id: i32, pool: &PgPool) -> HttpResponse {
        let project_exists = check_project_po_by_id(pool, id).await;
        match project_exists {
            Ok(_) => { 
                let delete_result = delete_project_po(pool, id).await;
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
            Err(e) => {
                println!("Delete error: {:?}", e); // Logging error
                HttpResponse::NotFound().json(MessageResponse {
                    success: false,
                    message: format!("Project with ID {} not found", id),
                })
            }
        }
    }
}
