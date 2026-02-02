//! Development server for kakeboor
//!
//! Starts the HTTP server for local development.

use kakeboor::{get_settings, url_patterns};
use reinhardt::core::tokio;
use reinhardt::server::serve;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Set settings module environment variable
    // SAFETY: This is safe because we're setting it before any other code runs
    unsafe {
        std::env::set_var("REINHARDT_SETTINGS_MODULE", "kakeboor.config.settings");
    }

    let _settings = get_settings();
    let router = url_patterns();

    let addr: SocketAddr = "127.0.0.1:8000".parse().expect("Invalid address");

    println!("Starting development server at http://127.0.0.1:8000/");
    println!("Quit the server with CONTROL-C.");

    // Run the server
    if let Err(e) = serve(addr, router).await {
        eprintln!("Server error: {}", e);
        std::process::exit(1);
    }
}
