// src/main.rs
use actix_web::{web, App, HttpServer};
use crate::api::run_api;

mod smart_contract;
mod governance;
mod collateralization;
mod multi_sig_wallet;
mod api;
mod events;
mod utils;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("Starting Pi Coin Stablecoin API...");
    run_api().await
}
