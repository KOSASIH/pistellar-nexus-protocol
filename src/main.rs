// Entry point for the PiStellar Nexus Core application

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

fn main() {
    // Initialize the PiStellar Nexus Core application
    let mut app = PiStellarNexusCore::new();

    // Initialize the Stellar protocol model
    let stellar_protocol = StellarProtocol::new();

    // Initialize the Pi Network model
    let pi_network = PiNetwork::new();

    // Initialize the cryptography utility functions
    let crypto_utils = CryptoUtils::new();

    // Initialize the network utility functions
    let network_utils = NetworkUtils::new();

    // Initialize the Stellar tool
    let stellar_tool = StellarTool::new();

    // Initialize the Pi tool
    let pi_tool = PiTool::new();

    // Start the PiStellar Nexus Core application
    app.start(stellar_protocol, pi_network, crypto_utils, network_utils, stellar_tool, pi_tool);
}
