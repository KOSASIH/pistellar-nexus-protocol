use actix_web::{web, App, HttpServer, middleware::Logger};
use crate::api::run_api;
use std::env;

mod smart_contract;
mod governance;
mod collateralization;
mod multi_sig_wallet;
mod api;
mod events;
mod utils;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Set up logging
    std::env::set_var("RUST_LOG", "actix_web=info,info");
    env_logger::init();

    println!("Starting Pi Coin Stablecoin API...");

    // Run the API server
    run_api().await
}

// Enhanced run_api function to include middleware and error handling
pub async fn run_api() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default()) // Enable logging middleware
            .route("/mint", web::post().to(api::mint))
            .route("/burn", web::post().to(api::burn))
            .route("/transfer", web::post().to(api::transfer))
            .route("/balance/{user}", web::get().to(api::get_balance))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
                                                    }
