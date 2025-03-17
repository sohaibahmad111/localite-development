use actix_web::{web, HttpResponse, Scope, get};

#[get("/list")]
async fn list_transactions() -> HttpResponse {
    // Placeholder - implement real transaction listing
    HttpResponse::Ok().json(serde_json::json!([
        {
            "id": 1,
            "sender_id": "user1",
            "receiver_id": "user2",
            "amount": 100.0,
            "purpose": "Tour guide",
            "timestamp": "2025-03-18T12:00:00"
        }
    ]))
}

pub fn transaction_routes() -> Scope {
    web::scope("/transactions")
        .service(list_transactions)
}