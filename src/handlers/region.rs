use actix_web::{web, HttpResponse};
use sqlx::PgPool;

use crate::{GetPSA, GetRegional, GetWitel, Region};





// region handlers
pub async fn get_regional(
    pool: web::Data<PgPool>,
    // user_id: web::Path<i32>
    user_id: web::Query<GetRegional>
) -> HttpResponse {
    Region::get_regional(user_id.user_id, &pool).await
}

pub async fn get_witel(
    pool: web::Data<PgPool>,
    // user_id: web::Path<i32>,
    // regional: web::Path<i32>,
    param: web::Query<GetWitel>
) -> HttpResponse {
    // let param = GetWitel{ user_id: user_id.into_inner(), regional: regional.into_inner() };
    Region::get_witel(param.into_inner(), &pool).await
}

pub async fn get_psa(
    pool: web::Data<PgPool>,
    // user_id: web::Path<i32>,
    // regional: web::Path<i32>,
    // witel: web::Path<i32>,
    param: web::Query<GetPSA>
) -> HttpResponse {
    // let param = GetPSA{ user_id: user_id.into_inner(), regional: regional.into_inner(), witel: witel.into_inner() };
    Region::get_psa(param.into_inner(), &pool).await
}