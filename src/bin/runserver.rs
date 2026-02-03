//! Development server for kakeboor
//!
//! Starts the HTTP server for local development.

use kakeboor::{get_settings, url_patterns};
use reinhardt::core::tokio;
use reinhardt::db::orm::reinitialize_database;
use reinhardt::db::DatabaseConnection;
use reinhardt::server::serve;
use std::net::SocketAddr;

/// Create database tables if they don't exist
async fn create_tables(conn: &DatabaseConnection) -> Result<(), Box<dyn std::error::Error>> {
    // Create categories table
    conn.execute(
        r#"
        CREATE TABLE IF NOT EXISTS categories (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            category_type TEXT NOT NULL,
            icon TEXT,
            color TEXT,
            created_at TEXT NOT NULL DEFAULT (datetime('now'))
        )
        "#,
        vec![],
    )
    .await?;

    // Create transactions table
    conn.execute(
        r#"
        CREATE TABLE IF NOT EXISTS transactions (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            amount INTEGER NOT NULL,
            category_id INTEGER NOT NULL,
            description TEXT NOT NULL,
            transaction_date TEXT NOT NULL,
            transaction_type TEXT NOT NULL,
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            updated_at TEXT NOT NULL DEFAULT (datetime('now')),
            FOREIGN KEY (category_id) REFERENCES categories(id)
        )
        "#,
        vec![],
    )
    .await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    // Set settings module environment variable
    // SAFETY: This is safe because we're setting it before any other code runs
    unsafe {
        std::env::set_var("REINHARDT_SETTINGS_MODULE", "kakeboor.config.settings");
    }

    let _settings = get_settings();

    // Initialize database with SQLite (mode=rwc creates file if not exists)
    let db_url = "sqlite:db.sqlite3?mode=rwc";
    if let Err(e) = reinitialize_database(db_url).await {
        eprintln!("Warning: Failed to initialize database: {}", e);
        eprintln!("Continuing without database...");
    } else {
        // Create tables if they don't exist
        match DatabaseConnection::connect(db_url).await {
            Ok(conn) => {
                if let Err(e) = create_tables(&conn).await {
                    eprintln!("Warning: Failed to create tables: {}", e);
                } else {
                    println!("Database tables ready.");
                }
            }
            Err(e) => {
                eprintln!("Warning: Failed to connect for table creation: {}", e);
            }
        }
    }

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
