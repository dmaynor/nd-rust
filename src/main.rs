use tracing_subscriber::{EnvFilter, fmt};
use nd_core::Settings; // Import Settings
use db::create_pool; // Import db pool creation function
use web::run_server; // Import web server run function

#[tokio::main] // Make main async
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing subscriber
    // Use `RUST_LOG=nd_rust=info` (or trace, debug, warn, error) to control level
    // Defaults to `info` if RUST_LOG is not set.
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));
    fmt::Subscriber::builder()
        .with_env_filter(filter)
        .init();

    tracing::info!("nd-rust starting");

    // Load configuration
    let settings = match Settings::new() {
        Ok(s) => {
            tracing::info!(?s, "Configuration loaded successfully");
            s
        }
        Err(e) => {
            tracing::error!(error = %e, "Failed to load configuration");
            eprintln!("Error loading configuration: {}", e);
            std::process::exit(1);
        }
    };

    // Create database connection pool
    let db_pool = match create_pool(&settings).await {
        Ok(pool) => {
            tracing::info!("Database connection pool created successfully");
            pool
        }
        Err(e) => {
            tracing::error!(error = %e, "Failed to create database pool");
            eprintln!("Error connecting to database: {}", e);
            // Depending on the error, might want to wait/retry or exit
            std::process::exit(1); 
        }
    };

    // Run the web server
    tracing::info!("Starting web server...");
    if let Err(e) = run_server(db_pool, &settings).await {
        tracing::error!(error = %e, "Web server failed");
        eprintln!("Web server error: {}", e);
        std::process::exit(1);
    }

    // Note: The server runs indefinitely, so this point might not be reached
    // unless the server is shut down gracefully later.
    tracing::info!("nd-rust shutting down");
    Ok(())
}
