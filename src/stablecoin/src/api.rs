// src/api.rs
use actix_web::{web, HttpResponse, Responder};

pub async fn run_api() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/mint", web::post().to(mint))
            .route("/burn", web::post().to(burn))
            .route("/transfer", web::post().to(transfer))
            .route("/balance/{user}", web::get().to(get_balance))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn mint() -> impl Responder {
    HttpResponse::Ok().body("Minting functionality")
}

async fn burn() -> impl Responder {
    HttpResponse::Ok().body("Burning functionality")
}

async fn transfer() -> impl Responder {
    HttpResponse::Ok().body("Transfer functionality")
}

async fn get_balance(web::Path(user): web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("Balance for user: {}", user))
}
