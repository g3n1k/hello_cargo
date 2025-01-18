use actix_web::HttpResponse;
use serde_json::json;
use sqlx::PgPool;
use crate::{get_psa_all, get_psa_by_region_ids, get_regional_all, get_regional_by_region_ids, get_user_credential, get_witel_all, get_witel_by_region_ids, ApiResponse, DataResponse, GetWitel, GetPSA, Region, Regional, Witel, PSA};


impl Region {
    pub async fn get_regional(user_id: i32, pool: &PgPool) -> HttpResponse {
        let user_info;
        match get_user_credential(pool, user_id).await {
            Ok(users) => {
                // println!("credential: {users:?}");
                user_info = users;
            },
            Err(_err) => {
                return HttpResponse::InternalServerError().json(DataResponse {
                    success: false,
                    data: json!([]),
                });
            },
        }

        let regional = if user_info.role == Some(format!("admin")) {
            get_regional_all(pool).await
        } else {
            get_regional_by_region_ids(pool, user_info.region_id).await
        };
    
        match regional { 
            Ok(data) =>  {
                HttpResponse::Ok().json(ApiResponse{ 
                    status: true, 
                    message: format!("Regional found"), 
                    data: Some(data) })
            }, 
            Err(err) => {
                HttpResponse::Ok().json(ApiResponse{ 
                    status: false, 
                    message: format!("Failed to get regional. {}", err), 
                    data: None::<Vec<Regional>>,
                })
            }
        }
    }

    pub async fn get_witel(req: GetWitel, pool: &PgPool) -> HttpResponse {
        let user_info;
        match get_user_credential(pool, req.user_id).await {
            Ok(users) => {
                // println!("credential: {users:?}");
                user_info = users;
            },
            Err(_err) => {
                return HttpResponse::InternalServerError().json(DataResponse {
                    success: false,
                    data: json!([]),
                });
            },
        }
    
        let witel = if user_info.role == Some(format!("admin")) {
            get_witel_all(pool, req.regional).await
        } else {
            get_witel_by_region_ids(pool, user_info.region_id, req.regional).await
        };
    
        match witel { 
            Ok(data) =>  {
                HttpResponse::Ok().json(ApiResponse{ 
                    status: true, 
                    message: format!("Witel found"), 
                    data: Some(data) })
            }, 
            Err(err) => {
                HttpResponse::Ok().json(ApiResponse{ 
                    status: false, 
                    message: format!("Failed to get witel. {}", err), 
                    data: None::<Vec<Witel>>,
                })
            }
        }
    }

    pub async fn get_psa(req: GetPSA, pool: &PgPool) -> HttpResponse {
        let user_info;
        match get_user_credential(pool, req.user_id).await {
            Ok(users) => {
                // println!("credential: {users:?}");
                user_info = users;
            },
            Err(_err) => {
                return HttpResponse::InternalServerError().json(DataResponse {
                    success: false,
                    data: json!([]),
                });
            },
        }
    
        let psa = if user_info.role == Some(format!("admin")) {
            get_psa_all(pool, req).await
        } else {
            get_psa_by_region_ids(pool, user_info, req).await
        };
    
        match psa { 
            Ok(data) =>  {
                HttpResponse::Ok().json(ApiResponse{ 
                    status: true, 
                    message: format!("PSA found"), 
                    data: Some(data) })
            }, 
            Err(err) => {
                HttpResponse::Ok().json(ApiResponse{ 
                    status: false, 
                    message: format!("Failed to get psa. {}", err), 
                    data: None::<Vec<PSA>>,
                })
            }
        }
    }
}


