use actix_web::{web, App, HttpServer, HttpResponse, Responder, middleware};
use serde_json::json;
use std::sync::Mutex;
use log::{info, error};

#[derive(Clone)]
struct AppState {
    user_count: Mutex<u32>,
}

/// Handler for the index route.
async fn index(data: web::Data<AppState>) -> impl Responder {
    let count = data.user_count.lock().unwrap();
    HttpResponse::Ok().json(json!({
        "message": "Welcome to the Dashboard!",
        "user_count": *count,
    }))
}

/// Handler for incrementing the user count.
async fn increment_user_count(data: web::Data<AppState>) -> impl Responder {
    let mut count = data.user_count.lock().unwrap();
    *count += 1;
    info!("User  count incremented to {}", *count);
    HttpResponse::Ok().json(json!({
        "user_count": *count,
    }))
}

/// Function to run the web dashboard server.
pub async fn run_web_dashboard() -> std::io::Result<()> {
    // Initialize application state with user count set to 0
    let state = AppState {
        user_count: Mutex::new(0),
    };

    // Start the HTTP server
    HttpServer::new(move || {
        App::new()
            .data(state.clone()) // Share state across requests
            .wrap(middleware::Logger::default()) // Enable logging middleware
            .route("/", web::get().to(index)) // Route for the index page
            .route("/increment", web::post().to(increment_user_count)) // Route for incrementing user count
    })
    .bind("127.0.0.1:8080")? // Bind to the specified address
    .run() // Run the server
    .await
}
