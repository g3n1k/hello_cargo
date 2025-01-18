// use actix_web::{web, HttpResponse, Responder};
// use bcrypt::{hash, DEFAULT_COST};
// use serde::{Deserialize, Serialize};
// use sqlx::{PgPool, Result};

// use crate::Login;

// use super::MessageResponse;

// #[derive(Serialize, Deserialize)]
// pub struct User {
//     id: i32,
//     username: String,
//     role: Option<String>,
// }

// #[derive(Serialize, Deserialize)]
// pub struct CreateUser {
//     username: String,
//     password: String,
//     region_id: i32,
//     role_id: i16,
// }

// #[derive(Serialize, Deserialize)]
// pub struct UpdateUser {
//     id: i32,
//     username: String,
//     password: String,
//     region_id: i32,
//     role_id: i16,
// }

// #[derive(Serialize, Deserialize)]
// pub struct DeleteUser {
//     id: i32
// }

// #[derive(Serialize, Deserialize, Debug)]
// pub struct UserInfo {
//     pub region_id: i32,
//     pub role: Option<String>,
// }

// pub async fn get_user_data(pool: web::Data<PgPool>) -> impl Responder {
//     let rows = sqlx::query_as!(User, "
//         SELECT 
//             a.id as id, username, b.role as role 
//         FROM public.users a
//         LEFT JOIN public.role b on a.role_id = b.id")
//         .fetch_all(pool.get_ref())
//         .await;

//     match rows {
//         Ok(rows) => {
//             HttpResponse::Ok().json(rows)
//         },
//         Err(_) => HttpResponse::InternalServerError().body("Database error"),
//     }
// }

// pub async fn create_user(req: web::Json<CreateUser>, pool: web::Data<PgPool>) -> impl Responder {
//     let username = &req.username;
//     let password = &req.password;
//     let region_id = &req.region_id;
//     let role_id = req.role_id;

//     // Query the database for the user
//     let user = sqlx::query_as!(Login,
//         "SELECT username, password FROM public.users WHERE username = $1",
//         username
//     )
//     .fetch_one(&**pool)
//     .await;

//     if let Ok(_) = user {
//         return HttpResponse::Conflict().json(MessageResponse {
//             success: false,
//             message: "User already exist".to_string(),
//         });
//     }

//     match hash(password, DEFAULT_COST) {
//         Ok(password_hash) => {
//             let insert = sqlx::query!("INSERT INTO public.users (username, password, region_id, role_id) VALUES ($1, $2, $3, $4) returning id",
//                 username, password_hash, region_id, role_id
//             )
//             .fetch_one(&**pool)
//             .await;
            
//             match insert {
//                 Ok(_id) => {
//                     HttpResponse::Ok().json(MessageResponse {
//                         success: true,
//                         message: "Insert data success".to_string(),
//                     })
//                 },
//                 Err(_e) => {
//                     HttpResponse::InternalServerError().json(MessageResponse {
//                         success: false,
//                         message: "Insert data failed".to_string(),
//                     })
//                 },
//             }
//         },
//         Err(_e) => {
//             HttpResponse::InternalServerError().json(MessageResponse {
//                 success: false,
//                 message: "An error occurred while processing your request".to_string(),
//             })
//         },
//     }
// }

// pub async fn update_user(req: web::Json<UpdateUser>, pool: web::Data<PgPool>) -> impl Responder {
//     let username = &req.username;
//     let password = &req.password;
//     let region_id = &req.region_id;
//     let role_id = &req.role_id;

//     match hash(password, DEFAULT_COST) {
//         Ok(password_hash) => {
//             let insert = sqlx::query!("
//                 UPDATE public.users 
//                 SET 
//                     username=$2, 
//                     password=$3, 
//                     region_id=$4,
//                     role_id=$5 
//                 WHERE id = $1
//                 returning id",
//                 req.id ,username, password_hash, region_id, role_id
//             )
//             .fetch_one(&**pool)
//             .await;

//             match insert {
//                 Ok(_id) => {
//                     HttpResponse::Ok().json(MessageResponse {
//                         success: true,
//                         message: "Insert data success".to_string(),
//                     })
//                 },
//                 Err(_e) => {
//                     HttpResponse::InternalServerError().json(MessageResponse {
//                         success: false,
//                         message: "Insert data failed".to_string(),
//                     })
//                 },
//             }
//         },
//         Err(_) => {
//             HttpResponse::InternalServerError().json(MessageResponse {
//                 success: false,
//                 message: "An error occurred while processing your request".to_string(),
//             })
//         },
//     }
// }

// pub async fn delete_user(req: web::Json<DeleteUser>, pool: web::Data<PgPool>) -> impl Responder {
//     let insert = sqlx::query!("
//         DELETE FROM public.users
//         WHERE id=$1
//         returning id",
//         req.id
//     )
//     .fetch_one(&**pool)
//     .await;

//     match insert {
//         Ok(_id) => {
//             HttpResponse::Ok().json(MessageResponse {
//                 success: true,
//                 message: "Delete data success".to_string(),
//             })
//         },
//         Err(_e) => {
//             HttpResponse::InternalServerError().json(MessageResponse {
//                 success: false,
//                 message: "Delete data failed".to_string(),
//             })
//         },
//     }
// }

// pub async fn get_users_by_id(pool: web::Data<PgPool>, user_id: i32) -> Result<UserInfo> {
//     sqlx::query_as!(UserInfo,
//         "
//         SELECT 
//             b.role as role, region_id
//         FROM public.users a
//         LEFT JOIN public.role b on a.role_id = b.id 
//         WHERE a.id=$1",
//         user_id
//     )
//     .fetch_one(&**pool)
//     .await
// }