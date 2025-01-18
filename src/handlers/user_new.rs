use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use crate::model::{ApiResponse, AreaAssigned, CreateUserRequest, LoginRequest, Team, UpdateUserRequest, User, UserAreaAssigned};

// User handlers
pub async fn create_user_new(
    pool: web::Data<PgPool>,
    request: web::Json<CreateUserRequest>
) -> HttpResponse {
    User::create_with_areas(&pool, request.into_inner()).await
}

pub async fn get_user_new_by_team_id(pool: web::Data<PgPool>, team_id: web::Path<i32>) -> HttpResponse {
    User::get_by_team_id(&pool, team_id.into_inner()).await
}

pub async fn get_user_new(pool: web::Data<PgPool>, user_id: web::Path<i32>) -> HttpResponse {
    User::get_by_user_id(&pool, user_id.into_inner()).await
}

pub async fn update_user_new(
    pool: web::Data<PgPool>,
    id: web::Path<i32>,
    request: web::Json<UpdateUserRequest>,
) -> HttpResponse {
    User::update_with_areas(&pool, id.into_inner(), request.into_inner()).await
}

pub async fn delete_user_new(pool: web::Data<PgPool>, id: web::Path<i32>) -> HttpResponse {
    User::delete(&pool, id.into_inner()).await
}

// Team handlers
pub async fn create_team(pool: web::Data<PgPool>, team: web::Json<String>) -> HttpResponse {
    Team::create(&pool, team.into_inner()).await
}

pub async fn get_all_teams(pool: web::Data<PgPool>) -> HttpResponse {
    Team::get_all(&pool).await
}

pub async fn get_teams_by_user_id(user_id: web::Path<i32>,pool: web::Data<PgPool>) -> HttpResponse {
    Team::get_by_user_id(user_id.into_inner(), &pool).await
}

pub async fn update_team(
    pool: web::Data<PgPool>,
    id: web::Path<i32>,
    team_name: web::Json<String>,
) -> HttpResponse {
    match Team::update(&pool, id.into_inner(), team_name.into_inner()).await {
        Ok(team) => HttpResponse::Ok().json(ApiResponse {
            status: true,
            message: "Team updated successfully".to_string(),
            data: Some(team),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse {
            status: false,
            message: format!("Failed to update team: {}", e),
            data: None::<Team>,
        }),
    }
}

pub async fn delete_team(pool: web::Data<PgPool>, id: web::Path<i32>) -> HttpResponse {
    match Team::delete(&pool, id.into_inner()).await {
        Ok(_) => HttpResponse::Ok().json(ApiResponse {
            status: true,
            message: "Team deleted successfully".to_string(),
            data: None::<()>,
        }),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse {
            status: false,
            message: format!("Failed to delete team: {}", e),
            data: None::<()>,
        }),
    }
}

// Area handlers
pub async fn create_area(pool: web::Data<PgPool>, area_name: web::Json<String>) -> HttpResponse {
    AreaAssigned::create(&pool, area_name.into_inner()).await
}

pub async fn get_all_areas(pool: web::Data<PgPool>) -> HttpResponse {
    match AreaAssigned::get_all(&pool).await {
        Ok(areas) => HttpResponse::Ok().json(ApiResponse {
            status: true,
            message: "Areas retrieved successfully".to_string(),
            data: Some(areas),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse {
            status: false,
            message: format!("Failed to retrieve areas: {}", e),
            data: None::<Vec<AreaAssigned>>,
        }),
    }
}

pub async fn update_area(
    pool: web::Data<PgPool>,
    id: web::Path<i32>,
    area_name: web::Json<String>,
) -> HttpResponse {
    match AreaAssigned::update(&pool, id.into_inner(), area_name.into_inner()).await {
        Ok(area) => HttpResponse::Ok().json(ApiResponse {
            status: true,
            message: "Area updated successfully".to_string(),
            data: Some(area),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse {
            status: false,
            message: format!("Failed to update area: {}", e),
            data: None::<AreaAssigned>,
        }),
    }
}

pub async fn delete_area(pool: web::Data<PgPool>, id: web::Path<i32>) -> HttpResponse {
    match AreaAssigned::delete(&pool, id.into_inner()).await {
        Ok(_) => HttpResponse::Ok().json(ApiResponse {
            status: true,
            message: "Area deleted successfully".to_string(),
            data: None::<()>,
        }),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse {
            status: false,
            message: format!("Failed to delete area: {}", e),
            data: None::<()>,
        }),
    }
}

// User Area Assigned handlers
pub async fn create_user_area(
    pool: web::Data<PgPool>,
    user_area: web::Json<UserAreaAssigned>,
) -> HttpResponse {
    UserAreaAssigned::create(&pool, user_area.into_inner()).await
}

pub async fn get_user_areas(pool: web::Data<PgPool>, user_id: web::Path<i32>) -> HttpResponse {
    UserAreaAssigned::get_by_user_id(&pool, user_id.into_inner()).await
}

pub async fn delete_user_area(
    pool: web::Data<PgPool>,
    params: web::Path<(i32, i32)>,
) -> HttpResponse {
    let (user_id, area_id) = params.into_inner();
    UserAreaAssigned::delete(&pool, user_id, area_id).await
}

pub async fn get_all_user_areas(pool: web::Data<PgPool>) -> HttpResponse {
    UserAreaAssigned::get_all(&pool).await
}

// Add this handler
pub async fn get_all_users_new(
    pool: web::Data<PgPool>
) -> HttpResponse { 
    // let user_id = request.into_inner().user_id;
    User::get_all_users(&pool).await
}

// Add this handler function
pub async fn login_user_new(
    pool: web::Data<PgPool>,
    login_req: web::Json<LoginRequest>,
) -> HttpResponse {
    User::login(&pool, login_req.into_inner()).await
} 