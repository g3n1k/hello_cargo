use actix_web::{web, HttpResponse, Responder};

use crate::*;




async fn index() -> impl Responder {
    HttpResponse::Ok().body("Welcome to the Actix web service")
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/", web::get().to(index))
        .route("/tests", web::get().to(index))
        .service(
            web::scope("/api/v1")

            //auth
            .service(
                web::scope("/auth")
                .route("/login", web::post().to(handlers::login_user_new))
            )

            // user
            .service(
                web::scope("/users-new")                    
                    .route("", web::post().to(handlers::create_user_new))
                    .route("", web::get().to(handlers::get_all_users_new))
                    .route("/team/{team_id}", web::get().to(handlers::get_user_new_by_team_id))
                    .route("/{user_id}", web::get().to(handlers::get_user_new))
                    .route("/{id}", web::put().to(handlers::update_user_new))
                    .route("/{id}", web::delete().to(handlers::delete_user_new))
            )

            // //project
            // .service(
            //     web::scope("/project")                    
            //         .route("", web::post().to(handlers::create_project))
            //         .route("/{user_id}", web::get().to(handlers::get_project))
            //         .route("", web::put().to(handlers::update_project))
            //         .route("/{id}", web::delete().to(handlers::delete_project))
            // )

            //wo project
            .service(
                web::scope("/project_wo")                    
                    .route("", web::post().to(handlers::create_project_wo))
                    .route("/{user_id}", web::get().to(handlers::get_project_wo))
                    .route("", web::put().to(handlers::update_project_wo))
                    .route("/{id}", web::delete().to(handlers::delete_project_wo))
            )

            //po project
            .service(
                web::scope("/project_po")                    
                    .route("", web::post().to(handlers::create_project_po))
                    .route("/{user_id}", web::get().to(handlers::get_project_po))
                    .route("", web::put().to(handlers::update_project_po))
                    .route("/{id}", web::delete().to(handlers::delete_project_po))
            )

            //progression_wo
            .service(
                web::scope("/progression_wo")                    
                    .route("", web::post().to(handlers::create_progression_wo))
                    .route("/{user_id}", web::get().to(handlers::get_progression_wo))
                    .route("", web::put().to(handlers::update_progression_wo))
                    .route("/{id}", web::delete().to(handlers::delete_progression_wo))
            )
            
            //progression_po
            .service(
                web::scope("/progression_po")
                    .route("", web::post().to(handlers::create_progression_po))
                    .route("/{user_id}", web::get().to(handlers::get_progression_po))
                    .route("", web::put().to(handlers::update_progression_po))
                    .route("/{id}", web::delete().to(handlers::delete_progression_po))
            )
            
            //region
            .service(
                web::scope("/region")
                    .route("/regional", web::get().to(handlers::get_regional))
                    .route("/witel", web::get().to(handlers::get_witel))
                    .route("/psa", web::get().to(handlers::get_psa))
            )
            
            //recon
            .service(
                web::scope("/recon")
                    .route("/get_po/{user_id}", web::get().to(handlers::get_recon_po))
                    .route("/{user_id}", web::get().to(handlers::get_recon))
                    .route("", web::post().to(handlers::create_recon))
                    .route("", web::put().to(handlers::update_recon))
                    .route("/{id}", web::delete().to(handlers::delete_recon))
            )
    
            // Team Routes
            .service(
            web::scope("/teams")
                .route("", web::post().to(handlers::create_team))
                .route("", web::get().to(handlers::get_all_teams))
                .route("{user_id}", web::get().to(handlers::get_teams_by_user_id))
                .route("/{id}", web::put().to(handlers::update_team))
                .route("/{id}", web::delete().to(handlers::delete_team))
            )
    
            // Area Assigned Routes
            .service(
            web::scope("/areas")
                .route("", web::post().to(handlers::create_area))
                .route("", web::get().to(handlers::get_all_areas))
                .route("/{id}", web::put().to(handlers::update_area))
                .route("/{id}", web::delete().to(handlers::delete_area))
            )
    
            // User Area Assigned Routes
            .service(
            web::scope("/user-areas")
                .route("", web::post().to(handlers::create_user_area))
                .route("/all", web::get().to(handlers::get_all_user_areas))
                .route("/{user_id}", web::get().to(handlers::get_user_areas))
                .route("/{user_id}/{area_id}", web::delete().to(handlers::delete_user_area))
            )

            // Phase
            .service(
                web::scope("/phase")
                    .route("", web::get().to(handlers::get_phase))    
                )

            // Status
            .service(
                web::scope("/status")
                    .route("", web::get().to(handlers::get_status))                    
                )

            // Status
            .service(
                web::scope("/project_type")
                    .route("", web::get().to(handlers::get_project_type))                    
                )

            // // Project Tracking
            // .service(
            //     web::scope("/project_tracking")
            //         .route("{user_id}", web::get().to(handlers::get_project_tracking))                    
            //         .route("", web::post().to(handlers::create_project_tracking))                    
            //         .route("", web::put().to(handlers::update_project_tracking))                    
            //         .route("{id}", web::delete().to(handlers::delete_project_tracking))                    
            //     )

            // Billing
            .service(
                web::scope("/billing")
                    .route("{user_id}", web::get().to(handlers::get_billing))
                    .route("", web::put().to(handlers::update_billing))
                )
        )
        ;
}