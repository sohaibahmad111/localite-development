use actix_web::{web, HttpResponse, Scope, get};
use crate::repository::Repository;

#[get("/profile")]
async fn get_profile(repo: web::Data<Repository>) -> HttpResponse {
    match repo.get_users() {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::InternalServerError().json(serde_json::json!({"error": "Failed to get users"})),
    }
}

pub fn user_routes() -> Scope {
    web::scope("/users")
        .service(get_profile)
}