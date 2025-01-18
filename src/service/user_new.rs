use actix_web::HttpResponse;
use bcrypt::{hash,  DEFAULT_COST}; 
use sqlx::PgPool;
use jsonwebtoken::{encode, Header, EncodingKey};
use chrono::{Utc, Duration};
use bcrypt::verify;
use crate::{get_pic, get_user_credential, model::User, repository, ApiResponse, AreaAssigned, Claims, CreateUserRequest, LoginRequest, LoginResponse, Team, UpdateUserRequest, UserAreaAssigned, UserCredential, UserDetail, PIC};


// User CRUD Operations
impl User {
    pub async fn create(pool: &PgPool, mut new_user: User) -> HttpResponse {
        // Check if user exists
        let existing_user = sqlx::query!(
            "SELECT id FROM users_new WHERE employee_id = $1",
            new_user.employee_id
        )
        .fetch_optional(pool)
        .await;

        match existing_user {
            Ok(Some(_)) => {
                return HttpResponse::BadRequest().json(ApiResponse {
                    status: false,
                    message: "User with this employee ID already exists".to_string(),
                    data: None::<User>,
                });
            }
            Err(e) => {
                return HttpResponse::InternalServerError().json(ApiResponse {
                    status: false,
                    message: format!("Database error: {}", e),
                    data: None::<User>,
                });
            }
            Ok(None) => {
                // Hash password
                match hash(new_user.password.as_bytes(), DEFAULT_COST) {
                    Ok(hashed) => {
                        new_user.password = hashed;
                        
                        // Create user
                        match sqlx::query_as!(
                            User,
                            r#"
                            INSERT INTO users_new (
                                username, password, full_name, email, phone_number, role_id,
                                area_assigned_id, team_id, employee_id, status, date_of_birth,
                                join_date, last_login, gender, address, profile_picture, notes,
                                created_at, updated_at
                            )
                            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, NOW(), NOW())
                            RETURNING *
                            "#,
                            new_user.username,
                            new_user.password,
                            new_user.full_name,
                            new_user.email,
                            new_user.phone_number,
                            new_user.role_id,
                            new_user.area_assigned_id,
                            new_user.team_id,
                            new_user.employee_id,
                            new_user.status,
                            new_user.date_of_birth,
                            new_user.join_date,
                            new_user.last_login,
                            new_user.gender,
                            new_user.address,
                            new_user.profile_picture,
                            new_user.notes,
                        )
                        .fetch_one(pool)
                        .await
                        {
                            Ok(user) => HttpResponse::Ok().json(ApiResponse {
                                status: true,
                                message: "User created successfully".to_string(),
                                data: Some(user),
                            }),
                            Err(e) => HttpResponse::InternalServerError().json(ApiResponse {
                                status: false,
                                message: format!("Failed to create user: {}", e),
                                data: None::<User>,
                            }),
                        }
                    }
                    Err(e) => HttpResponse::InternalServerError().json(ApiResponse {
                        status: false,
                        message: format!("Password hashing error: {}", e),
                        data: None::<User>,
                    }),
                }
            }
        }
    }

    pub async fn get_by_team_id(pool: &PgPool, team_id: i32) -> HttpResponse {
        match get_pic(pool, team_id).await {
            Ok(data) => {
                HttpResponse::Ok().json(ApiResponse {
                    status: true,
                    message: "User found".to_string(),
                    data: Some(data),
                })
            },
            Err(err) => {
                println!("error: {err}");
                HttpResponse::Ok().json(ApiResponse {
                    status: true,
                    message: "User Not found".to_string(),
                    data: None::<PIC>,
                })
            }
        }
    }

    pub async fn get_by_user_id(pool: &PgPool, user_id: i32) -> HttpResponse {
        match sqlx::query!(
            r#"
            SELECT 
                u.id,
                u.username,
                u.full_name,
                u.email,
                u.phone_number,
                u.employee_id,
                u.status,
                u.date_of_birth,
                u.last_login,
                u.join_date,
                u.gender,
                u.address,
                u.profile_picture,
                u.notes,
                u.created_at,
                u.updated_at,
                r.role,
                t.team_name,
                array_agg(DISTINCT aa.name) as assigned_areas
            FROM users_new u
            LEFT JOIN role r ON u.role_id = r.id
            LEFT JOIN team t ON u.team_id = t.id
            LEFT JOIN user_area_assigned uaa ON u.id = uaa.user_id
            LEFT JOIN region2 aa ON uaa.area_assigned_id = aa.id
            WHERE u.id = $1
            GROUP BY u.id, r.role, t.team_name
            "#,
            user_id
        )
        .fetch_optional(pool)
        .await {
            Ok(Some(row)) => {
                let user_detail = UserDetail {
                    id: row.id,
                    username: row.username,
                    full_name: row.full_name,
                    email: row.email,
                    phone_number: row.phone_number,
                    employee_id: row.employee_id,
                    status: row.status,
                    date_of_birth: row.date_of_birth,
                    join_date: row.join_date,
                    last_login: row.last_login,
                    gender: row.gender,
                    address: row.address,
                    profile_picture: row.profile_picture,
                    notes: row.notes,
                    created_at: row.created_at,
                    updated_at: row.updated_at,
                    role: row.role,
                    team_name: Some(row.team_name),
                    assigned_areas: Some(row.assigned_areas.unwrap_or_default()),
                };

                HttpResponse::Ok().json(ApiResponse {
                    status: true,
                    message: "User found".to_string(),
                    data: Some(user_detail),
                })
            },
            Ok(None) => HttpResponse::NotFound().json(ApiResponse {
                status: false,
                message: "User not found".to_string(),
                data: None::<UserDetail>,
            }),
            Err(e) => HttpResponse::InternalServerError().json(ApiResponse {
                status: false,
                message: format!("Database error: {}", e),
                data: None::<UserDetail>,
            }),
        }
    }

    pub async fn update(pool: &PgPool, user_id: i32, mut updated_user: User) -> HttpResponse {
        // Check if user exists
        let existing_user = sqlx::query!(
            "SELECT id FROM users_new WHERE id = $1",
            user_id
        )
        .fetch_optional(pool)
        .await;

        match existing_user {
            Ok(None) => {
                return HttpResponse::NotFound().json(ApiResponse {
                    status: false,
                    message: "User not found".to_string(),
                    data: None::<User>,
                });
            }
            Err(e) => {
                return HttpResponse::InternalServerError().json(ApiResponse {
                    status: false,
                    message: format!("Database error: {}", e),
                    data: None::<User>,
                });
            }
            Ok(Some(_)) => {
                // Hash new password if provided
                if !updated_user.password.is_empty() {
                    match hash(updated_user.password.as_bytes(), DEFAULT_COST) {
                        Ok(hashed) => updated_user.password = hashed,
                        Err(e) => {
                            return HttpResponse::InternalServerError().json(ApiResponse {
                                status: false,
                                message: format!("Password hashing error: {}", e),
                                data: None::<User>,
                            });
                        }
                    }
                }

                // Update user
                match sqlx::query_as!(
                    User,
                    r#"
                    UPDATE users_new SET
                        username = $1, password = $2, full_name = $3, email = $4,
                        phone_number = $5, role_id = $6, area_assigned_id = $7,
                        team_id = $8, employee_id = $9, status = $10,
                        date_of_birth = $11, join_date = $12, last_login = $13,
                        gender = $14, address = $15, profile_picture = $16,
                        notes = $17
                    WHERE id = $18
                    RETURNING *
                    "#,
                    updated_user.username,
                    updated_user.password,
                    updated_user.full_name,
                    updated_user.email,
                    updated_user.phone_number,
                    updated_user.role_id,
                    updated_user.area_assigned_id,
                    updated_user.team_id,
                    updated_user.employee_id,
                    updated_user.status,
                    updated_user.date_of_birth,
                    updated_user.join_date,
                    updated_user.last_login,
                    updated_user.gender,
                    updated_user.address,
                    updated_user.profile_picture,
                    updated_user.notes,
                    user_id
                )
                .fetch_one(pool)
                .await
                {
                    Ok(user) => HttpResponse::Ok().json(ApiResponse {
                        status: true,
                        message: "User updated successfully".to_string(),
                        data: Some(user),
                    }),
                    Err(e) => HttpResponse::InternalServerError().json(ApiResponse {
                        status: false,
                        message: format!("Failed to update user: {}", e),
                        data: None::<User>,
                    }),
                }
            }
        }
    }

    pub async fn delete(pool: &PgPool, user_id: i32) -> HttpResponse {
        // Check if user exists
        let existing_user = sqlx::query!(
            "SELECT id FROM users_new WHERE id = $1",
            user_id
        )
        .fetch_optional(pool)
        .await;

        match existing_user {
            Ok(None) => {
                HttpResponse::NotFound().json(ApiResponse {
                    status: false,
                    message: "User not found".to_string(),
                    data: None::<()>,
                })
            }
            Err(e) => {
                HttpResponse::InternalServerError().json(ApiResponse {
                    status: false,
                    message: format!("Database error: {}", e),
                    data: None::<()>,
                })
            }
            Ok(Some(_)) => {
                match sqlx::query!("DELETE FROM users_new WHERE id = $1", user_id)
                    .execute(pool)
                    .await
                {
                    Ok(_) => HttpResponse::Ok().json(ApiResponse {
                        status: true,
                        message: "User deleted successfully".to_string(),
                        data: None::<()>,
                    }),
                    Err(e) => HttpResponse::InternalServerError().json(ApiResponse {
                        status: false,
                        message: format!("Failed to delete user: {}", e),
                        data: None::<()>,
                    }),
                }
            }
        }
    } 

    pub async fn get_all_users(pool: &PgPool) -> HttpResponse {
        // let role;
        // let region_id;
        // if let Ok(crd) = get_user_credential(pool, user_id).await{
        //     role = crd.role;
        //     region_id = crd.region_id;
        // } else {
        //     return HttpResponse::InternalServerError().json(ApiResponse {
        //         status: false,
        //         message: format!("Can't get User Credential"),
        //         data: None::<Vec<UserDetail>>,
        //     });
        // };

        // let get = if role == Some(format!("admin")) {
        //     repository::user::get_user_area_all(pool).await
        // } else {
        //     repository::user::get_user_area_by_regions(pool, region_id).await
        // };

        let get = repository::repo_user::get_user_area_all(pool).await;
        match get {
            Ok(users) => {                
                // println!("users: {:?}", users);
                HttpResponse::Ok().json(ApiResponse {
                    status: true,
                    message: format!("Successfully retrieved {} users", users.len()),
                    data: Some(users),
                })
            },
            Err(e) => HttpResponse::InternalServerError().json(ApiResponse {
                status: false,
                message: format!("Database error: {}", e),
                data: None::<Vec<UserDetail>>,
            }),
        }
    }

    pub async fn create_with_areas(pool: &PgPool, request: CreateUserRequest) -> HttpResponse {
        let mut tx = match pool.begin().await {
            Ok(tx) => tx,
            Err(e) => return HttpResponse::InternalServerError().json(ApiResponse {
                status: false,
                message: format!("Failed to start transaction: {}", e),
                data: None::<UserDetail>,
            }),
        };

        // Check existing user
        if let Ok(Some(_)) = sqlx::query!(
            "SELECT id FROM users_new WHERE employee_id = $1",
            request.user.employee_id
        )
        .fetch_optional(&mut *tx)
        .await
        {
            return HttpResponse::BadRequest().json(ApiResponse {
                status: false,
                message: "User with this employee ID already exists".to_string(),
                data: None::<UserDetail>,
            });
        }

        // Hash password
        let hashed_password = match hash(request.user.password.as_bytes(), DEFAULT_COST) {
            Ok(hashed) => hashed,
            Err(e) => return HttpResponse::InternalServerError().json(ApiResponse {
                status: false,
                message: format!("Password hashing error: {}", e),
                data: None::<UserDetail>,
            }),
        };

        // Create user
        let created_user = match sqlx::query_as!(
            User,
            r#"
            INSERT INTO users_new (
                username, password, full_name, email, phone_number, role_id,
                team_id, employee_id, status, date_of_birth,
                join_date, last_login, gender, address, profile_picture, notes,
                created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, NOW(), NOW())
            RETURNING *
            "#,
            request.user.username,
            hashed_password,
            request.user.full_name,
            request.user.email,
            request.user.phone_number,
            request.user.role_id,
            request.user.team_id,
            request.user.employee_id,
            request.user.status,
            request.user.date_of_birth,
            request.user.join_date,
            request.user.last_login,
            request.user.gender,
            request.user.address,
            request.user.profile_picture,
            request.user.notes,
        )
        .fetch_one(&mut *tx)
        .await
        {
            Ok(user) => user,
            Err(e) => {
                let _ = tx.rollback().await;
                return HttpResponse::InternalServerError().json(ApiResponse {
                    status: false,
                    message: format!("Failed to create user: {}", e),
                    data: None::<UserDetail>,
                });
            }
        };

        // Insert area assignments
        for area_id in &request.area_ids {
            if let Err(e) = sqlx::query!(
                r#"
                INSERT INTO user_area_assigned (user_id, area_assigned_id)
                VALUES ($1, $2)
                "#,
                created_user.id,
                area_id
            )
            .execute(&mut *tx)
            .await
            {
                let _ = tx.rollback().await;
                return HttpResponse::InternalServerError().json(ApiResponse {
                    status: false,
                    message: format!("Failed to assign area {}: {}", area_id, e),
                    data: None::<UserDetail>,
                });
            }
        }

        // Commit transaction
        if let Err(e) = tx.commit().await {
            return HttpResponse::InternalServerError().json(ApiResponse {
                status: false,
                message: format!("Failed to commit transaction: {}", e),
                data: None::<UserDetail>,
            });
        }

        // Fetch complete user details with joined data
        match sqlx::query_as!(UserDetail,
            r#"
            SELECT 
                u.id,
                u.username,
                u.full_name,
                u.email,
                u.phone_number,
                u.employee_id,
                u.status,
                u.date_of_birth,
                u.join_date,
                u.last_login,
                u.gender,
                u.address,
                u.profile_picture,
                u.notes,
                u.created_at,
                u.updated_at,
                r.role,
                t.team_name,
                array_agg(DISTINCT aa.name) as assigned_areas
            FROM users_new u
            LEFT JOIN role r ON u.role_id = r.id
            LEFT JOIN team t ON u.team_id = t.id
            LEFT JOIN user_area_assigned uaa ON u.id = uaa.user_id
            LEFT JOIN region2 aa ON uaa.area_assigned_id = aa.id
            WHERE u.id = $1
            GROUP BY u.id, r.role, t.team_name
            "#,
            created_user.id
        )
        .fetch_one(pool)
        .await
        {
            Ok(user_detail) => {                
                HttpResponse::Ok().json(ApiResponse {
                    status: true,
                    message: "User created successfully with area assignments".to_string(),
                    data: Some(user_detail),
                })
            }
            Err(e) => HttpResponse::InternalServerError().json(ApiResponse {
                status: false,
                message: format!("Failed to fetch user details: {}", e),
                data: None::<UserDetail>,
            }),
        }
    }

    pub async fn update_with_areas(pool: &PgPool, user_id: i32, request: UpdateUserRequest) -> HttpResponse {
        // Start transaction
        let mut tx = match pool.begin().await {
            Ok(tx) => tx,
            Err(e) => return HttpResponse::InternalServerError().json(ApiResponse {
                status: false,
                message: format!("Failed to start transaction: {}", e),
                data: None::<User>,
            }),
        };

        // Update user
        let updated_user = match sqlx::query_as!(
            User,
            r#"
            UPDATE users_new SET
                username = $1, full_name = $2, email = $3,
                phone_number = $4, role_id = $5, area_assigned_id = $6,
                team_id = $7, employee_id = $8, status = $9,
                date_of_birth = $10, join_date = $11, last_login = $12,
                gender = $13, address = $14, profile_picture = $15,
                notes = $16, updated_at = NOW()
            WHERE id = $17
            RETURNING *
            "#,
            request.user.username,
            request.user.full_name,
            request.user.email,
            request.user.phone_number,
            request.user.role_id,
            request.user.area_assigned_id,
            request.user.team_id,
            request.user.employee_id,
            request.user.status,
            request.user.date_of_birth,
            request.user.join_date,
            request.user.last_login,
            request.user.gender,
            request.user.address,
            request.user.profile_picture,
            request.user.notes,
            user_id
        )
        .fetch_one(&mut *tx)
        .await
        {
            Ok(user) => user,
            Err(e) => {
                let _ = tx.rollback().await;
                return HttpResponse::InternalServerError().json(ApiResponse {
                    status: false,
                    message: format!("Failed to update user: {}", e),
                    data: None::<User>,
                });
            }
        };

        // Delete existing area assignments
        if let Err(e) = sqlx::query!(
            "DELETE FROM user_area_assigned WHERE user_id = $1",
            user_id
        )
        .execute(&mut *tx)
        .await
        {
            let _ = tx.rollback().await;
            return HttpResponse::InternalServerError().json(ApiResponse {
                status: false,
                message: format!("Failed to remove existing area assignments: {}", e),
                data: None::<User>,
            });
        }

        // Insert new area assignments
        for area_id in request.area_ids {
            if let Err(e) = sqlx::query!(
                r#"
                INSERT INTO user_area_assigned (user_id, area_assigned_id)
                VALUES ($1, $2)
                "#,
                user_id,
                area_id
            )
            .execute(&mut *tx)
            .await
            {
                let _ = tx.rollback().await;
                return HttpResponse::InternalServerError().json(ApiResponse {
                    status: false,
                    message: format!("Failed to assign areas: {}", e),
                    data: None::<User>,
                });
            }
        }

        // Commit transaction
        if let Err(e) = tx.commit().await {
            return HttpResponse::InternalServerError().json(ApiResponse {
                status: false,
                message: format!("Failed to commit transaction: {}", e),
                data: None::<User>,
            });
        }

        HttpResponse::Ok().json(ApiResponse {
            status: true,
            message: "User updated successfully with new area assignments".to_string(),
            data: Some(updated_user),
        })
    }

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
                let expiration = Utc::now()
                    .checked_add_signed(Duration::hours(24))
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

                HttpResponse::Ok().json(ApiResponse {
                    status: true,
                    message: "Login successful".to_string(),
                    data: Some(LoginResponse {
                        token,
                        user: user_detail,
                    }),
                })
            }
            _ => HttpResponse::Unauthorized().json(ApiResponse {
                status: false,
                message: "Invalid credentials".to_string(),
                data: None::<LoginResponse>,
            }),
        }
    }
}

// Team CRUD Operations with API Response
impl Team {
    pub async fn create(pool: &PgPool, team_name: String) -> HttpResponse {
        // Check if team exists
        let existing_team = sqlx::query!(
            "SELECT id FROM team WHERE team_name = $1",
            team_name
        )
        .fetch_optional(pool)
        .await;

        match existing_team {
            Ok(Some(_)) => {
                HttpResponse::BadRequest().json(ApiResponse {
                    status: false,
                    message: "Team with this name already exists".to_string(),
                    data: None::<Team>,
                })
            }
            Err(e) => {
                HttpResponse::InternalServerError().json(ApiResponse {
                    status: false,
                    message: format!("Database error: {}", e),
                    data: None::<Team>,
                })
            }
            Ok(None) => {
                match sqlx::query_as!(
                    Team,
                    "INSERT INTO team (team_name) VALUES ($1) RETURNING *",
                    team_name
                )
                .fetch_one(pool)
                .await
                {
                    Ok(team) => HttpResponse::Ok().json(ApiResponse {
                        status: true,
                        message: "Team created successfully".to_string(),
                        data: Some(team),
                    }),
                    Err(e) => HttpResponse::InternalServerError().json(ApiResponse {
                        status: false,
                        message: format!("Failed to create team: {}", e),
                        data: None::<Team>,
                    }),
                }
            }
        }
    }

    pub async fn get_all(pool: &PgPool) -> HttpResponse {
        match repository::get_team_all(pool).await
        {
            Ok(teams) => HttpResponse::Ok().json(ApiResponse {
                status: true,
                message: "Teams retrieved successfully".to_string(),
                data: Some(teams),
            }),
            Err(e) => HttpResponse::InternalServerError().json(ApiResponse {
                status: false,
                message: format!("Failed to retrieve teams: {}", e),
                data: None::<Vec<Team>>,
            }),
        }
    }

    pub async fn get_by_user_id(user_id: i32, pool: &PgPool) -> HttpResponse {
        let user_info ;
        if let Ok(crd) = get_user_credential(pool, user_id).await{
            // println!("credential: {crd:?}");
            user_info = crd;
        } else {
            return HttpResponse::InternalServerError().json(ApiResponse {
                status: false,
                message: format!("Can't get User Credential"),
                data: None::<Vec<UserCredential>>,
            });
        };

        let team = if user_info.role == Some(format!("admin")) {
            repository::get_team_all(pool).await
        } else {
            repository::get_team_by_regions(pool, user_info.region_id).await
        };

        match team {
            Ok(teams) => HttpResponse::Ok().json(ApiResponse {
                status: true,
                message: "Teams retrieved successfully".to_string(),
                data: Some(teams),
            }),
            Err(e) => HttpResponse::InternalServerError().json(ApiResponse {
                status: false,
                message: format!("Failed to retrieve teams: {}", e),
                data: None::<Vec<Team>>,
            }),
        }
    }

    pub async fn update(pool: &PgPool, team_id: i32, team_name: String) -> Result<Team, sqlx::Error> {
        let team = sqlx::query_as!(
            Team,
            "UPDATE team SET team_name = $1 WHERE id = $2 RETURNING *",
            team_name,
            team_id
        )
        .fetch_one(pool)
        .await?;

        Ok(team)
    }

    pub async fn delete(pool: &PgPool, team_id: i32) -> Result<(), sqlx::Error> {
        sqlx::query!("DELETE FROM team WHERE id = $1", team_id)
            .execute(pool)
            .await?;

        Ok(())
    }
}

// Area Assigned CRUD Operations with API Response
impl AreaAssigned {
    pub async fn create(pool: &PgPool, area_name: String) -> HttpResponse {
        // Check if area exists
        let existing_area = sqlx::query!(
            "SELECT id FROM area_assigned WHERE area_name = $1",
            area_name
        )
        .fetch_optional(pool)
        .await;

        match existing_area {
            Ok(Some(_)) => {
                HttpResponse::BadRequest().json(ApiResponse {
                    status: false,
                    message: "Area with this name already exists".to_string(),
                    data: None::<AreaAssigned>,
                })
            }
            Err(e) => {
                HttpResponse::InternalServerError().json(ApiResponse {
                    status: false,
                    message: format!("Database error: {}", e),
                    data: None::<AreaAssigned>,
                })
            }
            Ok(None) => {
                match sqlx::query_as!(
                    AreaAssigned,
                    "INSERT INTO area_assigned (area_name) VALUES ($1) RETURNING *",
                    area_name
                )
                .fetch_one(pool)
                .await
                {
                    Ok(area) => HttpResponse::Ok().json(ApiResponse {
                        status: true,
                        message: "Area created successfully".to_string(),
                        data: Some(area),
                    }),
                    Err(e) => HttpResponse::InternalServerError().json(ApiResponse {
                        status: false,
                        message: format!("Failed to create area: {}", e),
                        data: None::<AreaAssigned>,
                    }),
                }
            }
        }
    }

    pub async fn get_all(pool: &PgPool) -> Result<Vec<AreaAssigned>, sqlx::Error> {
        let areas = sqlx::query_as!(AreaAssigned, "SELECT * FROM area_assigned")
            .fetch_all(pool)
            .await?;

        Ok(areas)
    }

    pub async fn update(
        pool: &PgPool,
        area_id: i32,
        area_name: String,
    ) -> Result<AreaAssigned, sqlx::Error> {
        let area = sqlx::query_as!(
            AreaAssigned,
            "UPDATE area_assigned SET area_name = $1 WHERE id = $2 RETURNING *",
            area_name,
            area_id
        )
        .fetch_one(pool)
        .await?;

        Ok(area)
    }

    pub async fn delete(pool: &PgPool, area_id: i32) -> Result<(), sqlx::Error> {
        sqlx::query!("DELETE FROM area_assigned WHERE id = $1", area_id)
            .execute(pool)
            .await?;

        Ok(())
    }
}

impl UserAreaAssigned {
    pub async fn create(pool: &PgPool, user_area: UserAreaAssigned) -> HttpResponse {
        // Check if mapping already exists
        let existing = sqlx::query!(
            "SELECT user_id FROM user_area_assigned WHERE user_id = $1 AND area_assigned_id = $2",
            user_area.user_id,
            user_area.area_assigned_id
        )
        .fetch_optional(pool)
        .await;

        match existing {
            Ok(Some(_)) => {
                HttpResponse::BadRequest().json(ApiResponse {
                    status: false,
                    message: "This user-area mapping already exists".to_string(),
                    data: None::<UserAreaAssigned>,
                })
            }
            Err(e) => {
                HttpResponse::InternalServerError().json(ApiResponse {
                    status: false,
                    message: format!("Database error: {}", e),
                    data: None::<UserAreaAssigned>,
                })
            }
            Ok(None) => {
                match sqlx::query_as!(
                    UserAreaAssigned,
                    r#"
                    INSERT INTO user_area_assigned (user_id, area_assigned_id)
                    VALUES ($1, $2)
                    RETURNING *
                    "#,
                    user_area.user_id,
                    user_area.area_assigned_id
                )
                .fetch_one(pool)
                .await
                {
                    Ok(mapping) => HttpResponse::Ok().json(ApiResponse {
                        status: true,
                        message: "User-area mapping created successfully".to_string(),
                        data: Some(mapping),
                    }),
                    Err(e) => HttpResponse::InternalServerError().json(ApiResponse {
                        status: false,
                        message: format!("Failed to create user-area mapping: {}", e),
                        data: None::<UserAreaAssigned>,
                    }),
                }
            }
        }
    }

    pub async fn get_by_user_id(pool: &PgPool, user_id: i32) -> HttpResponse {
        match sqlx::query_as!(
            UserAreaAssigned,
            "SELECT * FROM user_area_assigned WHERE user_id = $1",
            user_id
        )
        .fetch_all(pool)
        .await
        {
            Ok(mappings) => HttpResponse::Ok().json(ApiResponse {
                status: true,
                message: "User area mappings retrieved successfully".to_string(),
                data: Some(mappings),
            }),
            Err(e) => HttpResponse::InternalServerError().json(ApiResponse {
                status: false,
                message: format!("Failed to retrieve user area mappings: {}", e),
                data: None::<Vec<UserAreaAssigned>>,
            }),
        }
    }

    pub async fn delete(pool: &PgPool, user_id: i32, area_id: i32) -> HttpResponse {
        match sqlx::query!(
            "DELETE FROM user_area_assigned WHERE user_id = $1 AND area_assigned_id = $2",
            user_id,
            area_id
        )
        .execute(pool)
        .await
        {
            Ok(result) => {
                if result.rows_affected() > 0 {
                    HttpResponse::Ok().json(ApiResponse {
                        status: true,
                        message: "User-area mapping deleted successfully".to_string(),
                        data: None::<()>,
                    })
                } else {
                    HttpResponse::NotFound().json(ApiResponse {
                        status: false,
                        message: "User-area mapping not found".to_string(),
                        data: None::<()>,
                    })
                }
            }
            Err(e) => HttpResponse::InternalServerError().json(ApiResponse {
                status: false,
                message: format!("Failed to delete user-area mapping: {}", e),
                data: None::<()>,
            }),
        }
    }

    pub async fn get_all(pool: &PgPool) -> HttpResponse {
        match sqlx::query_as!(
            UserAreaAssigned,
            "SELECT * FROM user_area_assigned"
        )
        .fetch_all(pool)
        .await
        {
            Ok(mappings) => HttpResponse::Ok().json(ApiResponse {
                status: true,
                message: "All user-area mappings retrieved successfully".to_string(),
                data: Some(mappings),
            }),
            Err(e) => HttpResponse::InternalServerError().json(ApiResponse {
                status: false,
                message: format!("Failed to retrieve user-area mappings: {}", e),
                data: None::<Vec<UserAreaAssigned>>,
            }),
        }
    }
} 
