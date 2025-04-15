use sqlx::postgres::PgPoolOptions;
use nd_core::Settings;
use std::time::Duration;
use ipnetwork::IpNetwork;

mod models;
pub use models::{Device, DeviceStatus, Interface};

pub use sqlx::postgres::PgPool;

#[derive(Debug, thiserror::Error)]
pub enum DbError {
    #[error("Database configuration missing")]
    ConfigMissing,
    #[error("Database query failed: {0}")]
    QueryFailed(sqlx::Error),
    #[error("Device not found")]
    NotFound,
    #[error("Failed to map row to struct: {0}")]
    MappingError(String),
}

// Implement From<sqlx::Error> for DbError
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

// --- Device Storage Functions (Refactored) ---

/// Inserts a new device or updates an existing one based on ip_address.
/// Returns the resulting Device record (including generated ID and timestamps).
pub async fn upsert_device(pool: &PgPool, device_data: &Device) -> Result<Device, DbError> {
    let row = sqlx::query!(
        r#"
        INSERT INTO devices (
            hostname, ip_address, sys_name, sys_descr, vendor, model, 
            os_version, serial_number, status, last_seen
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9::device_status, $10)
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
        RETURNING 
            id, hostname, ip_address, sys_name, sys_descr, vendor, model, 
            os_version, serial_number, 
            status::text as "status: Option<String>", -- Select enum as text with type hint
            last_seen, created_at, updated_at
        "#,
        device_data.hostname,
        device_data.ip_address,
        device_data.sys_name,
        device_data.sys_descr,
        device_data.vendor,
        device_data.model,
        device_data.os_version,
        device_data.serial_number,
        // Pass status as String to avoid macro type issue with enums
        device_data.status.as_ref().map(|s| format!("{:?}", s).to_lowercase()) as Option<String>,
        device_data.last_seen
    )
    .fetch_one(pool)
    .await?;

    // Map the result row (handle Option<Option<String>> for status)
    let status_opt_opt: Option<Option<String>> = row.status;
    let status: Option<DeviceStatus> = status_opt_opt
        .flatten() // Flatten Option<Option<String>> to Option<String>
        .map(|s| DeviceStatus::try_from(s))
        .transpose()
        .map_err(DbError::MappingError)?;

    Ok(Device {
        id: row.id,
        hostname: row.hostname,
        ip_address: row.ip_address,
        sys_name: row.sys_name,
        sys_descr: row.sys_descr,
        vendor: row.vendor,
        model: row.model,
        os_version: row.os_version,
        serial_number: row.serial_number,
        status, // Use the mapped status
        last_seen: row.last_seen,
        created_at: row.created_at,
        updated_at: row.updated_at,
    })
}

/// Retrieves a device by its unique IP address.
pub async fn get_device_by_ip(pool: &PgPool, ip_address: IpNetwork) -> Result<Device, DbError> {
    let row = sqlx::query!(
        r#"SELECT 
              id, hostname, ip_address, sys_name, sys_descr, vendor, model, 
              os_version, serial_number, 
              status::text as "status: Option<String>", 
              last_seen, created_at, updated_at 
           FROM devices WHERE ip_address = $1"#,
        ip_address
    )
    .fetch_one(pool)
    .await?; 
    
    // Map the result row (handle Option<Option<String>> for status)
    let status_opt_opt: Option<Option<String>> = row.status;
    let status: Option<DeviceStatus> = status_opt_opt
        .flatten() // Flatten Option<Option<String>> to Option<String>
        .map(|s| DeviceStatus::try_from(s))
        .transpose()
        .map_err(DbError::MappingError)?;

    Ok(Device {
        id: row.id,
        hostname: row.hostname,
        ip_address: row.ip_address,
        sys_name: row.sys_name,
        sys_descr: row.sys_descr,
        vendor: row.vendor,
        model: row.model,
        os_version: row.os_version,
        serial_number: row.serial_number,
        status, // Use the mapped status
        last_seen: row.last_seen,
        created_at: row.created_at,
        updated_at: row.updated_at,
    })
}

/// Retrieves a list of all devices.
pub async fn list_devices(pool: &PgPool) -> Result<Vec<Device>, DbError> {
    let rows = sqlx::query!(
        r#"SELECT 
              id, hostname, ip_address, sys_name, sys_descr, vendor, model, 
              os_version, serial_number, 
              status::text as "status: Option<String>", 
              last_seen, created_at, updated_at 
           FROM devices ORDER BY hostname, ip_address"#
    )
    .fetch_all(pool)
    .await?;

    let mut devices = Vec::with_capacity(rows.len());
    for row in rows {
        // Map the result row (handle Option<Option<String>> for status)
        let status_opt_opt: Option<Option<String>> = row.status;
        let status: Option<DeviceStatus> = status_opt_opt
            .flatten() // Flatten Option<Option<String>> to Option<String>
            .map(|s| DeviceStatus::try_from(s))
            .transpose()
            .map_err(DbError::MappingError)?;

        devices.push(Device {
            id: row.id,
            hostname: row.hostname,
            ip_address: row.ip_address,
            sys_name: row.sys_name,
            sys_descr: row.sys_descr,
            vendor: row.vendor,
            model: row.model,
            os_version: row.os_version,
            serial_number: row.serial_number,
            status, // Use the mapped status
            last_seen: row.last_seen,
            created_at: row.created_at,
            updated_at: row.updated_at,
        });
    }
    Ok(devices)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_compiles() {
        assert!(true);
    }
}
