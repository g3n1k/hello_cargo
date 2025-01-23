use actix_web::HttpResponse;
use sqlx::PgPool;

use crate::{get_user_credential, repository, ApiResponse, BillingHighlightView, Highlight, ProgressionPOHighlightView, ProjectHighlightView, UserManagementHighlightView};


impl Highlight {
    pub async fn get_project_highlight(pool: &PgPool, user_id: i32) -> HttpResponse {
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
                data: None::<Vec<Highlight>>,
            });
        };

        let highlight = if role == Some(format!("admin")) {
            repository::repo_highlight::get_project_highlight_all(pool).await
        } else {
            repository::repo_highlight::get_project_highlight_by_regions(pool, region_id).await
        };

        match highlight {
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
                    data: None::<Vec<ProjectHighlightView>>,
                })
            }
        }
    }

    pub async fn get_progression_po_highlight(pool: &PgPool, user_id: i32) -> HttpResponse {
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
                data: None::<Vec<Highlight>>,
            });
        };

        let highlight = if role == Some(format!("admin")) {
            repository::repo_highlight::get_progression_po_highlight_all(pool).await
        } else {
            repository::repo_highlight::get_progression_po_highlight_by_regions(pool, region_id).await
        };

        match highlight {
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
                    data: None::<Vec<ProgressionPOHighlightView>>,
                })
            }
        }
    }

    pub async fn get_progression_wo_highlight(pool: &PgPool, user_id: i32) -> HttpResponse {
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
                data: None::<Vec<Highlight>>,
            });
        };

        let highlight = if role == Some(format!("admin")) {
            repository::repo_highlight::get_progression_wo_highlight_all(pool).await
        } else {
            repository::repo_highlight::get_progression_wo_highlight_by_regions(pool, region_id).await
        };

        match highlight {
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
                    data: None::<Vec<ProgressionPOHighlightView>>,
                })
            }
        }
    }

    pub async fn get_recon_highlight(pool: &PgPool, user_id: i32) -> HttpResponse {
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
                data: None::<Vec<Highlight>>,
            });
        };

        let highlight = if role == Some(format!("admin")) {
            repository::repo_highlight::get_recon_highlight_all(pool).await
        } else {
            repository::repo_highlight::get_recon_highlight_by_regions(pool, region_id).await
        };

        match highlight {
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
                    data: None::<Vec<ProgressionPOHighlightView>>,
                })
            }
        }
    }

    pub async fn get_billing_highlight(pool: &PgPool, user_id: i32) -> HttpResponse {
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
                data: None::<Vec<Highlight>>,
            });
        };

        let highlight = if role == Some(format!("admin")) {
            repository::repo_highlight::get_billing_highlight_all(pool).await
        } else {
            repository::repo_highlight::get_billing_highlight_by_regions(pool, region_id).await
        };

        match highlight {
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
                    data: None::<Vec<BillingHighlightView>>,
                })
            }
        }
    }

    pub async fn get_user_management_highlight(pool: &PgPool, user_id: i32) -> HttpResponse {
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
                data: None::<Vec<Highlight>>,
            });
        };

        let highlight = if role == Some(format!("admin")) {
            repository::repo_highlight::get_user_management_highlight_all(pool).await
        } else {
            repository::repo_highlight::get_user_management_highlight_by_regions(pool, region_id).await
        };

        match highlight {
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
                    data: None::<Vec<UserManagementHighlightView>>,
                })
            }
        }
    }
}
