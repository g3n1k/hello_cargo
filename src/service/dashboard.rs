use actix_web::HttpResponse;
use sqlx::PgPool;

use crate::{repository, ApiResponse, DailyProgressionView, Dashboard, ProgressionUpdatesView, ProjectHighlightView, RegionalDistributionView, SummaryProgressionView};


impl Dashboard {
    pub async fn get_summary_project(pool: &PgPool) -> HttpResponse {
        let highlight = repository::repo_dashboard::get_summary_project_all(pool).await;

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

    pub async fn get_summary_progress(pool: &PgPool) -> HttpResponse {
        let highlight = repository::repo_dashboard::get_summary_progression_all(pool).await;

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
                    data: None::<Vec<SummaryProgressionView>>,
                })
            }
        }
    }

    pub async fn get_daily_progress(pool: &PgPool) -> HttpResponse {
        let highlight = repository::repo_dashboard::get_daily_progression_all(pool).await;

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
                    data: None::<Vec<DailyProgressionView>>,
                })
            }
        }
    }

    pub async fn get_progression_updates(pool: &PgPool) -> HttpResponse {
        let highlight = repository::repo_dashboard::get_progression_updates_all(pool).await;

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
                    data: None::<Vec<ProgressionUpdatesView>>,
                })
            }
        }
    }

    pub async fn get_regional_distribution(pool: &PgPool) -> HttpResponse {
        let highlight = repository::repo_dashboard::get_regional_distribution_all(pool).await;

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
                    data: None::<Vec<RegionalDistributionView>>,
                })
            }
        }
    }
}
