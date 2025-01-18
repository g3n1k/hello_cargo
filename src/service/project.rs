use actix_web::HttpResponse;
use serde_json::json;
use sqlx::PgPool;

use crate::{check_project, check_project_by_id, check_project_tracking, get_region_by, get_user_credential, insert_project, repository::{self, create_project_tracking, delete_project, update_project}, ApiResponse, CreateProject, CreateProjectTracking, DataResponse, GetProject, MessageResponse, Project, ProjectView, UpdateProject};


impl Project {
    pub async fn create_project(req: CreateProject, pool: &PgPool) -> HttpResponse {
        // Query the database for the user
        let project = check_project(pool, &req).await;
        if let Ok(_) = project {
            return HttpResponse::Conflict().json(MessageResponse {
                success: false,
                message: "WO/PO already exist".to_string(),
            });
        }
    
        let region;
        if let Ok(r) = get_region_by(pool, req.regional,  req.witel, req.psa).await {
            region=r;
        } else {
            return HttpResponse::InternalServerError().json(MessageResponse {
                success: false,
                message: "Region not found".to_string(),
            });
        }
    
        let region_id = region.id;
        let insert = insert_project(pool, &req, region_id).await;            
        match insert {
            Ok(project) => {
                //Project Tracking
                if let Some(id) = project.id {
                    let pt = CreateProjectTracking{ project_id: id, stage: format!("WO Creation") };
                    let check_project_tracking = check_project_tracking(pool, &pt).await;
                    if let Ok(_) = check_project_tracking {
                        return HttpResponse::InternalServerError().json(MessageResponse {
                            success: false,
                            message: "Project tracking already exist".to_string(),
                        });
                    }
                    let project_tracking = create_project_tracking(pool, &pt).await;
                    if let Err(e) = project_tracking {
                        println!("error insert project tracking: {e}");
                        return HttpResponse::InternalServerError().json(MessageResponse {
                            success: false,
                            message: "Failed when inserting project tracking".to_string(),
                        });
                    }
                }

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
    
    pub async fn get_project(user_id: i32, pool: &PgPool) -> HttpResponse {
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
                data: None::<Vec<GetProject>>,
            });
        };

        let projects = if role == Some(format!("admin")) {
            repository::repo_project::get_project_all(pool).await
        } else {
            repository::repo_project::get_project_by_regions(pool, region_id).await
        };

        match projects {
            Ok(project) => {
                let empty: Vec<ProjectView> = vec![];
                let data = serde_json::to_value(project).unwrap_or(json!(empty));
                HttpResponse::Ok().json(DataResponse {
                    success: true,
                    data,
                })
            }, 
            Err(err) => {
                HttpResponse::Ok().json(ApiResponse{ 
                    status: false, 
                    message: format!("Error {}", err), 
                    data: None::<Vec<ProjectView>>,
                })
            }
        }
    }

    pub async fn update(req: UpdateProject, pool: &PgPool) -> HttpResponse {
      
        let id = match req.id {
            Some(id) => id,
            None => return HttpResponse::BadRequest().json(MessageResponse {
                success: false,
                message: "Project ID is required".to_string(),
            }),
        };
    
        let project_exists: Result<Project, sqlx::Error> = check_project_by_id(pool, id).await;    
        let project ;
        if let Ok(p) = project_exists {
            project = p;
        } else {
            return HttpResponse::NotFound().json(MessageResponse {
                success: false,
                message: format!("Project with ID {} not found", id),
            });
        }
    
        let region;
        if let Ok(r) = get_region_by(&pool, req.regional, req.witel, req.psa).await {
            region = r;
        } else {
            return HttpResponse::InternalServerError().json(MessageResponse {
                success: false,
                message: "Region not found".to_string(),
            });
        }
    
        let region_id = region.id;
        let update = update_project(pool, &req, region_id).await;    
        match update {
            Ok(project_id) => {
                let po_old = project.po_id;
                let po_new = req.po_id;
                if (po_old.is_none() || po_old == Some(format!(""))) && (po_new.is_some() && po_new != Some(format!(""))) {
                    //Project Tracking
                    if let Some(id) = project_id.id {
                        let pt = CreateProjectTracking{ project_id: id, stage: format!("PO Creation") };
                        let check_project_tracking = check_project_tracking(pool, &pt).await;
                        if let Ok(_) = check_project_tracking {
                            return HttpResponse::InternalServerError().json(MessageResponse {
                                success: false,
                                message: "Project tracking already exist".to_string(),
                            });
                        }
                        let project_tracking = create_project_tracking(pool, &pt).await;
                        if let Err(e) = project_tracking {
                            println!("error insert project tracking: {e}");
                            return HttpResponse::InternalServerError().json(MessageResponse {
                                success: false,
                                message: "Failed when inserting project tracking".to_string(),
                            });
                        }
                    }
                }

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
        let project_exists = check_project_by_id(pool, id).await;
        match project_exists {
            Ok(_) => { 
                let delete_result = delete_project(pool, id).await;
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
