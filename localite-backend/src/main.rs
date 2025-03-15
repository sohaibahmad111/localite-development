use actix_web::http::header;
use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

mod db;
mod models;
mod repository;
mod schema;

use crate::repository::get_users;

#[actix_web::main]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, Localite Backend!")
}

async fn users_handler() -> Result<HttpResponse, actix_web::Error> {
    let users = web::block(|| get_users())
        .await?;

    Ok(HttpResponse::Ok().json(users))
}

#[actix_web::main]
use actix_web::http::header;

async fn main() -> std::io::Result<()> {
    // Load environment variables from .env file
    dotenvy::dotenv().ok();
    
    println!("Starting Localite Backend server at http://127.0.0.1:8080");
    
    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            .allowed_header(header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .wrap(cors)
            .route("/", web::get().to(hello))
            .route("/users", web::get().to(users_handler))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}