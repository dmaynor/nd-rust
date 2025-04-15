use sqlx::postgres::{PgPool, PgPoolOptions};
use nd_core::Settings;
use std::time::Duration;

mod models;
pub use models::{Device, DeviceStatus, Interface}; // Be explicit for clarity

// Re-export pool for easier use in other crates
// pub use sqlx::PgPool; // Removed redundant re-export

#[derive(Debug, thiserror::Error)]
pub enum DbError {
    #[error("Database configuration missing")]
    ConfigMissing,
    #[error("Failed to connect to database: {0}")]
    ConnectionFailed(#[from] sqlx::Error),
}

pub async fn create_pool(settings: &Settings) -> Result<PgPool, DbError> {
    let db_url = settings
        .database
        .as_ref()
        .and_then(|db_settings| db_settings.url.as_deref())
        .ok_or(DbError::ConfigMissing)?;

    tracing::info!(url = %mask_url(db_url), "Connecting to database");

    PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(db_url)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Database connection failed");
            DbError::ConnectionFailed(e)
        })
}

// Helper to mask password in URL for logging
fn mask_url(url: &str) -> String {
    match url::Url::parse(url) {
        Ok(mut parsed_url) => {
            if parsed_url.password().is_some() {
                let _ = parsed_url.set_password(Some("********"));
            }
            parsed_url.to_string()
        }
        Err(_) => "<invalid url>".to_string(), // Don't log original if parse fails
    }
}

#[cfg(test)]
mod tests {
    // We'll add tests later that require a running DB
    // For now, just ensure the code compiles
    #[test]
    fn it_compiles() {
        assert!(true);
    }
}
