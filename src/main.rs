mod lib;
mod models;
mod utils;
mod tools;

use lib::PiStellarNexusCore;
use models::stellar::StellarProtocol;
use models::pi::PiNetwork;
use utils::crypto::CryptoUtils;
use utils::network::NetworkUtils;
use tools::stellar_tool::StellarTool;
use tools::pi_tool::PiTool;
use log::{info, error};
use simplelog::{Config, LevelFilter, SimpleLogger};

fn main() {
    // Initialize logging
    SimpleLogger::init(LevelFilter::Info, Config::default()).unwrap();
    info!("Starting PiStellar Nexus Core application...");

    // Initialize the PiStellar Nexus Core application
    let mut app = match PiStellarNexusCore::new() {
        Ok(app) => app,
        Err(e) => {
            error!("Failed to initialize PiStellar Nexus Core: {}", e);
            return;
        }
    };

    // Initialize the Stellar protocol model
    let stellar_protocol = match StellarProtocol::new() {
        Ok(protocol) => protocol,
        Err(e) => {
            error!("Failed to initialize Stellar Protocol: {}", e);
            return;
        }
    };

    // Initialize the Pi Network model
    let pi_network = match PiNetwork::new() {
        Ok(network) => network,
        Err(e) => {
            error!("Failed to initialize Pi Network: {}", e);
            return;
        }
    };

    // Initialize the cryptography utility functions
    let crypto_utils = match CryptoUtils::new() {
        Ok(utils) => utils,
        Err(e) => {
            error!("Failed to initialize Crypto Utils: {}", e);
            return;
        }
    };

    // Initialize the network utility functions
    let network_utils = match NetworkUtils::new() {
        Ok(utils) => utils,
        Err(e) => {
            error!("Failed to initialize Network Utils: {}", e);
            return;
        }
    };

    // Initialize the Stellar tool
    let stellar_tool = match StellarTool::new() {
        Ok(tool) => tool,
        Err(e) => {
            error!("Failed to initialize Stellar Tool: {}", e);
            return;
        }
    };

    // Initialize the Pi tool
    let pi_tool = match PiTool::new() {
        Ok(tool) => tool,
        Err(e) => {
            error!("Failed to initialize Pi Tool: {}", e);
            return;
        }
    };

    // Start the PiStellar Nexus Core application
    if let Err(e) = app.start(stellar_protocol, pi_network, crypto_utils, network_utils, stellar_tool, pi_tool) {
        error!("Application failed to start: {}", e);
    } else {
        info!("PiStellar Nexus Core application started successfully.");
    }
        }
