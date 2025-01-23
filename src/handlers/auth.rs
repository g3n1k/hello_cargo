use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use crate::{model::LoginRequest, service};


// Add this handler function
pub async fn login(
    pool: web::Data<PgPool>,
    login_req: web::Json<LoginRequest>,
) -> HttpResponse {
    service::auth::login(&pool, login_req.into_inner()).await
} 

pub async fn logout() -> HttpResponse {
    service::auth::logout().await
} 