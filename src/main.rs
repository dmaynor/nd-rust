use tracing_subscriber::{EnvFilter, fmt};

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
    println!("Hello, world!");
    tracing::info!("Application finished");
}
