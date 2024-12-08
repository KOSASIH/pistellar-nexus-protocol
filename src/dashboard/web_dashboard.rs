use actix_web::{web, App, HttpServer, HttpResponse, Responder, middleware};
use serde_json::json;
use std::sync::Mutex;

#[derive(Clone)]
struct AppState {
    user_count: Mutex<u32>,
}

async fn index(data: web::Data<AppState>) -> impl Responder {
    let count = data.user_count.lock().unwrap();
    HttpResponse::Ok().json(json!({
        "message": "Welcome to the Dashboard!",
        "user_count": *count,
    }))
}

async fn increment_user_count(data: web::Data<AppState>) -> impl Responder {
    let mut count = data.user_count.lock().unwrap();
    *count += 1;
    HttpResponse::Ok().json(json!({
        "user_count": *count,
    }))
}

pub async fn run_web_dashboard() -> std::io::Result<()> {
    let state = AppState {
        user_count: Mutex::new(0),
    };

    HttpServer::new(move || {
        App::new()
            .data(state.clone())
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(index))
            .route("/increment", web::post().to(increment_user_count))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
