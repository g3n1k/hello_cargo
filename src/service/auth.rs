use actix_web::HttpResponse;
use bcrypt::verify;
use chrono::{Duration, Utc};
use cookie::time::OffsetDateTime;
use jsonwebtoken::{encode, Header, EncodingKey};
use sqlx::PgPool;

use crate::{ApiResponse, Claims, LoginRequest, LoginResponse, UserDetail};


pub async fn login(pool: &PgPool, login_req: LoginRequest) -> HttpResponse {        
    // Find user by username
    let user = match sqlx::query!(
        r#"
        SELECT u.*, r.role, t.team_name
        FROM users_new u
        LEFT JOIN role r ON u.role_id = r.id
        LEFT JOIN team t ON u.team_id = t.id
        WHERE u.username = $1
        "#,
        login_req.username
    )
    .fetch_optional(pool)
    .await
    {
        Ok(Some(user)) => user,
        Ok(None) => {
            return HttpResponse::Unauthorized().json(ApiResponse {
                status: false,
                message: "Invalid credentials".to_string(),
                data: None::<LoginResponse>,
            })
        }
        Err(e) => {
            return HttpResponse::InternalServerError().json(ApiResponse {
                status: false,
                message: format!("Database error: {}", e),
                data: None::<LoginResponse>,
            })
        }
    };

    // Verify password
    match verify(login_req.password.as_bytes(), &user.password) {
        Ok(is_valid) if is_valid => {
            // Generate JWT token
            // let duration = cookie::time::Duration::minutes(30);
            let duration = OffsetDateTime::now_utc().checked_add(cookie::time::Duration::minutes(1));
            let expiration = Utc::now()
                .checked_add_signed(Duration::minutes(30))
                .expect("valid timestamp")
                .timestamp();
            

            let claims = Claims {
                sub: user.id,
                username: user.username.clone(),
                exp: expiration,
            };

            let token = encode(
                &Header::default(),
                &claims,
                &EncodingKey::from_secret(
                    std::env::var("JWT_SECRET")
                        .unwrap_or_else(|_| "default_secret".to_string())
                        .as_bytes(),
                ),
            )
            .unwrap_or_else(|_| "".to_string());

            // Get assigned areas
            let areas = sqlx::query!(
                r#"
                SELECT array_agg(aa.name) as areas
                FROM user_area_assigned uaa
                JOIN region2 aa ON uaa.area_assigned_id = aa.id
                WHERE uaa.user_id = $1
                GROUP BY uaa.user_id
                "#,
                user.id
            )
            .fetch_optional(pool)
            .await
            .unwrap_or(None);
        
            let mut cookie = cookie::Cookie::build("token", token.clone())
            .http_only(true) // Penting: Cookie hanya dapat diakses oleh HTTP, mencegah XSS
            .secure(false) // Penting: Cookie hanya dikirim melalui HTTPS (WAJIB di produksi)
            // .same_site(cookie::SameSite::None) // Sangat disarankan untuk mencegah CSRF
            // .max_age(duration)
            // .expires(duration)
            .path("/")
            .finish();

            cookie.set_expires(duration);

            let user_detail = UserDetail {
                id: user.id,
                username: user.username,
                full_name: user.full_name,
                email: user.email,
                phone_number: user.phone_number,
                employee_id: user.employee_id,
                status: user.status,
                date_of_birth: user.date_of_birth,
                join_date: user.join_date,
                last_login: user.last_login,
                gender: user.gender,
                address: user.address,
                profile_picture: user.profile_picture,
                notes: user.notes,
                created_at: user.created_at,
                updated_at: user.updated_at,
                role: user.role,
                team_name: Some(user.team_name),
                assigned_areas: Some(areas.and_then(|a| a.areas).unwrap_or_default()),
            };

            HttpResponse::Ok()
                .cookie(cookie)
                .json(ApiResponse {
                    status: true,
                    message: "Login successful".to_string(),
                    data: Some(LoginResponse {
                        token,
                        user: user_detail,
                    }),
                }
            )
        }
        _ => HttpResponse::Unauthorized().json(ApiResponse {
            status: false,
            message: "Invalid credentials".to_string(),
            data: None::<LoginResponse>,
        }),
    }
}

pub async fn logout() -> HttpResponse {
    let cookie = cookie::Cookie::build("token", "-") // Nilai kosong
        .expires(OffsetDateTime::now_utc() + cookie::time::Duration::seconds(1)) // Kadaluarsa di masa lalu
        .http_only(true)
        .secure(false) // Pastikan ini true di produksi (HTTPS)
        // .same_site(cookie::SameSite::None)
        .path("/")
        .finish();

    HttpResponse::Ok().cookie(cookie).body("Logout Berhasil")
}