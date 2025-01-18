use std::fmt::format;

use actix_web::HttpResponse;
use sqlx::PgPool;

use crate::{check_recon, check_recon_by_id, get_po_recon_all, get_user_credential, insert_recon, repo_recon::{delete_recon, update_recon}, repository, ApiResponse, Billing, CreateBilling, CreateRecon, ReconPo, ReconView, Reconciliation};

use super:: MessageResponse;


impl Reconciliation {
    pub async fn get_po(pool: &PgPool, user_id: i32) -> HttpResponse {
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
                data: None::<Vec<Reconciliation>>,
            });
        };

        let recon = if role == Some(format!("admin")) {
            repository::repo_recon::get_po_recon_all(pool).await
        } else {
            repository::repo_recon::get_po_recon_by_regions(pool, region_id).await
        };

        match recon {
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
                    data: None::<Vec<ReconPo>>,
                })
            }
        }
    }

    pub async fn create(pool: &PgPool, req: CreateRecon) -> HttpResponse {
        // Query the database for the user
        let project = check_recon(pool, &req).await;
        if let Ok(_) = project {
            return HttpResponse::Conflict().json(MessageResponse {
                success: false,
                message: "Recon already exist".to_string(),
            });
        }

        let insert = insert_recon(pool, &req).await;            
        match insert {
            Ok(project) => {                
                //create billing automatically
                let bill_par = CreateBilling{ recon_id: Some(project.id), status: Some(format!("RECON")) };
                let _ = Billing::insert(pool, bill_par).await;

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
    
    pub async fn get(pool: &PgPool, user_id: i32) -> HttpResponse {
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
                data: None::<Vec<Reconciliation>>,
            });
        };

        let recon = if role == Some(format!("admin")) {
            repository::repo_recon::get_recon_all(pool).await
        } else {
            repository::repo_recon::get_recon_by_regions(pool, region_id).await
        };

        match recon {
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
                    data: None::<Vec<ReconView>>,
                })
            }
        }
    }

    pub async fn update(pool: &PgPool, req: Reconciliation) -> HttpResponse {
        let project_exists: Result<Reconciliation, sqlx::Error> = check_recon_by_id(pool, req.id).await;    
        if let Err(_) = project_exists {
            return HttpResponse::NotFound().json(MessageResponse {
                success: false,
                message: format!("Recon with ID {} not found", req.id),
            });
        }
    
        let update = update_recon(pool, &req).await;    
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
    
    pub async fn delete(pool: &PgPool, id: i32) -> HttpResponse {
        let project_exists = check_recon_by_id(pool, id).await;
        match project_exists {
            Ok(_) => { 
                let delete_result = delete_recon(pool, id).await;
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
