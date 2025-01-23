use actix_web::HttpResponse;
use sqlx::PgPool;

use crate::{MessageResponse, check_billing, check_billing_by_id, get_user_credential, insert_billing, repo_billing::update_billing, repository, ApiResponse, Billing, CreateBilling, ReconView};


impl Billing {
    pub async fn get(pool: &PgPool, user_id: i32) -> HttpResponse {
        let role;
        let region_id;
        if let Ok(crd) = get_user_credential(pool, user_id).await{
            role = crd.role;
            region_id = crd.region_id;
        } else {
            return HttpResponse::InternalServerError().json(ApiResponse {
                status: false,
                message: format!("Can't get User Credential"),
                data: None::<Vec<Billing>>,
            });
        };

        let billing = if role == Some(format!("admin")) {
            repository::repo_billing::get_billing_all(pool).await
        } else {
            repository::repo_billing::get_billing_by_regions(pool, region_id).await
        };

        match billing {
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

    pub async fn update(pool: &PgPool, req: Billing) -> HttpResponse {
        let project_exists: Result<Billing, sqlx::Error> = check_billing_by_id(pool, req.id).await;    
        if let Err(_) = project_exists {
            return HttpResponse::NotFound().json(MessageResponse {
                success: false,
                message: format!("Recon with ID {} not found", req.id),
            });
        }
    
        let update = update_billing(pool, &req).await;    
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

    pub async fn insert(pool: &PgPool, req: CreateBilling) -> HttpResponse {
        let project_exists: Result<Billing, sqlx::Error> = check_billing(pool, &req).await;    
        if let Ok(_) = project_exists {
            return HttpResponse::NotFound().json(MessageResponse {
                success: false,
                message: format!("Billing already exist"),
            });
        }
    
        let insert = insert_billing(pool, &req).await;    
        match insert {
            Ok(_project_id) => {                
                HttpResponse::Ok().json(MessageResponse {
                    success: true,
                    message: "Insert data success".to_string(),
                })
            }
            Err(e) => {
                println!("Insert error: {:?}", e); // Logging error
                HttpResponse::InternalServerError().json(MessageResponse {
                    success: false,
                    message: "Insert data failed".to_string(),
                })
            }
        }
    }
}
