use actix_web::{web, App, HttpServer, http::header}; 
use middleware::Auth;
use sqlx::PgPool;
use wfm_be::*;
use std::env;
use actix_cors::Cors; 




#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url).await.expect("Failed to connect to database");

    let url = "0.0.0.0:8080";
    println!("wfm running in: {url}");

    HttpServer::new(move || {
        let cors = Cors::permissive()
        .allow_any_origin() 
        // .allowed_origin("http://192.168.1.2:5173")
        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
        .allowed_headers(vec![
           header::AUTHORIZATION,
           header::ACCEPT,
           header::CONTENT_TYPE,
            ])
            .max_age(3600);

        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(cors)
            .wrap(Auth)
            .configure(config)
        })
    .bind(url)?
    .run()
    .await
}