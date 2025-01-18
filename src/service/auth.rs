use actix_web::{web, HttpResponse, Responder};
use bcrypt::verify;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::PgPool;





#[derive(Serialize)]
pub struct MessageResponse {
    pub success: bool,
    pub message: String,
}

#[derive(Serialize)]
pub struct DataResponse {
    pub success: bool,
    pub data: Value,
}

#[derive(Serialize, Deserialize)]
pub struct Login {
    pub username: String,
    pub password: String,
}

pub async fn login(login_req: web::Json<Login>, pool: web::Data<PgPool>) -> impl Responder {
    let username = &login_req.username;
    let password = &login_req.password;

    // Query the database for the user
    let user = sqlx::query_as!(Login,
        "SELECT username, password FROM public.users WHERE username = $1",
        username
    )
    .fetch_one(&**pool)
    .await;

    match user {
        Ok(user) => {
            // Compare the password with the hash in the database
            if verify(password, &user.password).unwrap_or(false) {
                HttpResponse::Ok().json(MessageResponse {
                    success: true,
                    message: "Login successful".to_string(),
                })
            } else {
                HttpResponse::Unauthorized().json(MessageResponse {
                    success: false,
                    message: "Invalid credentials".to_string(),
                })
            }
        }
        Err(_) => {
            // Handle database error
            HttpResponse::InternalServerError().json(MessageResponse {
                success: false,
                message: "An error occurred while processing your request".to_string(),
            })
        }
    }
}