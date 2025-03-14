use actix_web::{web, App, HttpResponse, HttpServer, Responder};

mod models;
mod repository;
mod schema;

use crate::repository::get_users;

async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, Localite Backend!")
}

async fn users_handler() -> impl Responder {
    let users = get_users();
    web::Json(users)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from .env file
    dotenvy::dotenv().ok();
    
    println!("Starting Localite Backend server at http://127.0.0.1:8080");
    
    HttpServer::new(|| 
        App::new()
            .route("/", web::get().to(hello))
            .route("/users", web::get().to(users_handler))
    )
    .bind("127.0.0.1:8080")?
    .run()
    .await
}