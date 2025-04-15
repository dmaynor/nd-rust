use sqlx::postgres::{PgPool, PgPoolOptions};
use nd_core::Settings;
use std::time::Duration;
use ipnetwork::IpNetwork; // Needed for get_device_by_ip
use uuid::Uuid; // Needed for get_device_by_ip

mod models;
pub use models::{Device, DeviceStatus, Interface};

pub use sqlx::PgPool;

#[derive(Debug, thiserror::Error)]
pub enum DbError {
    #[error("Database configuration missing")]
    ConfigMissing,
    #[error("Database query failed: {0}")]
    QueryFailed(sqlx::Error), // Use specific error variant
    #[error("Device not found")]
    NotFound,
}

// Map sqlx query errors
impl From<sqlx::Error> for DbError {
    fn from(e: sqlx::Error) -> Self {
        match e {
            sqlx::Error::RowNotFound => DbError::NotFound,
            _ => DbError::QueryFailed(e),
        }
    }
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
            DbError::from(e) 
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
        Err(_) => "<invalid url>".to_string(),
    }
}

// --- Device Storage Functions ---

/// Inserts a new device or updates an existing one based on ip_address.
/// Returns the resulting Device record (including generated ID and timestamps).
pub async fn upsert_device(pool: &PgPool, device_data: &Device) -> Result<Device, DbError> {
    // Simple example: Assume device_data has necessary fields populated except id/timestamps
    // More robust implementation would handle partial updates
    let result = sqlx::query_as!(
        Device,
        r#"
        INSERT INTO devices (
            hostname, ip_address, sys_name, sys_descr, vendor, model, 
            os_version, serial_number, status, last_seen
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        ON CONFLICT (ip_address) DO UPDATE SET
            hostname = EXCLUDED.hostname,
            sys_name = EXCLUDED.sys_name,
            sys_descr = EXCLUDED.sys_descr,
            vendor = EXCLUDED.vendor,
            model = EXCLUDED.model,
            os_version = EXCLUDED.os_version,
            serial_number = EXCLUDED.serial_number,
            status = EXCLUDED.status,
            last_seen = EXCLUDED.last_seen,
            updated_at = NOW()
        RETURNING *
        "#,
        device_data.hostname,
        device_data.ip_address,
        device_data.sys_name,
        device_data.sys_descr,
        device_data.vendor,
        device_data.model,
        device_data.os_version,
        device_data.serial_number,
        device_data.status as Option<DeviceStatus>, // Cast enum for sqlx macro
        device_data.last_seen
    )
    .fetch_one(pool)
    .await?;

    Ok(result)
}

/// Retrieves a device by its unique IP address.
pub async fn get_device_by_ip(pool: &PgPool, ip_address: IpNetwork) -> Result<Device, DbError> {
    let device = sqlx::query_as!(Device, "SELECT * FROM devices WHERE ip_address = $1", ip_address)
        .fetch_one(pool)
        .await?;
    Ok(device)
}

/// Retrieves a list of all devices.
pub async fn list_devices(pool: &PgPool) -> Result<Vec<Device>, DbError> {
    let devices = sqlx::query_as!(Device, "SELECT * FROM devices ORDER BY hostname, ip_address")
        .fetch_all(pool)
        .await?;
    Ok(devices)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_compiles() {
        assert!(true);
    }
}
