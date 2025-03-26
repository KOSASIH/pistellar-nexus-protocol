use actix_web::{web, HttpResponse, Responder, HttpServer, App, post, get};
use serde::Deserialize;
use std::sync::Mutex;
use std::collections::HashMap;

#[derive(Deserialize)]
struct MintRequest {
    user: String,
    amount: u64,
}

#[derive(Deserialize)]
struct BurnRequest {
    user: String,
    amount: u64,
}

#[derive(Deserialize)]
struct TransferRequest {
    from: String,
    to: String,
    amount: u64,
}

// Shared state for the API
struct AppState {
    balances: Mutex<HashMap<String, u64>>, // User balances
}

pub async fn run_api() -> std::io::Result<()> {
    let state = web::Data::new(AppState {
        balances: Mutex::new(HashMap::new()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .route("/mint", web::post().to(mint))
            .route("/burn", web::post().to(burn))
            .route("/transfer", web::post().to(transfer))
            .route("/balance/{user}", web::get().to(get_balance))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn mint(data: web::Json<MintRequest>, state: web::Data<AppState>) -> impl Responder {
    let mut balances = state.balances.lock().unwrap();
    let user_balance = balances.entry(data.user.clone()).or_insert(0);
    *user_balance += data.amount;

    HttpResponse::Ok().body(format!("Minted {} Pi Coins to {}", data.amount, data.user))
}

async fn burn(data: web::Json<BurnRequest>, state: web::Data<AppState>) -> impl Responder {
    let mut balances = state.balances.lock().unwrap();
    let user_balance = balances.entry(data.user.clone()).or_insert(0);

    if *user_balance < data.amount {
        return HttpResponse::BadRequest().body("Insufficient balance to burn");
    }

    *user_balance -= data.amount;
    HttpResponse::Ok().body(format!("Burned {} Pi Coins from {}", data.amount, data.user))
}

async fn transfer(data: web::Json<TransferRequest>, state: web::Data<AppState>) -> impl Responder {
    let mut balances = state.balances.lock().unwrap();
    let from_balance = balances.entry(data.from.clone()).or_insert(0);
    let to_balance = balances.entry(data.to.clone()).or_insert(0);

    if *from_balance < data.amount {
        return HttpResponse::BadRequest().body("Insufficient balance for transfer");
    }

    *from_balance -= data.amount;
    *to_balance += data.amount;

    HttpResponse::Ok().body(format!("Transferred {} Pi Coins from {} to {}", data.amount, data.from, data.to))
}

async fn get_balance(web::Path(user): web::Path<String>, state: web::Data<AppState>) -> impl Responder {
    let balances = state.balances.lock().unwrap();
    let balance = balances.get(&user).unwrap_or(&0);
    HttpResponse::Ok().body(format!("Balance for user {}: {}", user, balance))
    }
