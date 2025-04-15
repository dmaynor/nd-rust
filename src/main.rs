use tracing_subscriber::{EnvFilter, fmt};
use nd_core::Settings; // Import Settings

fn main() {
    // Initialize tracing subscriber
    // Use `RUST_LOG=nd_rust=info` (or trace, debug, warn, error) to control level
    // Defaults to `info` if RUST_LOG is not set.
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));
    fmt::Subscriber::builder()
        .with_env_filter(filter)
        .init();

    tracing::info!("Application starting");

    // Load configuration
    match Settings::new() {
        Ok(settings) => {
            tracing::info!(?settings, "Configuration loaded successfully");
            // Optionally use settings.log_level here later to configure tracing further
        }
        Err(e) => {
            tracing::error!(error = %e, "Failed to load configuration");
            // Decide how to handle config errors - exit? use defaults?
            eprintln!("Error loading configuration: {}", e);
            std::process::exit(1);
        }
    }

    println!("Hello, world!");
    tracing::info!("Application finished");
}
