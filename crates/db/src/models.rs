use time::OffsetDateTime;
use uuid::Uuid;
use ipnetwork::IpNetwork;
use sqlx::{FromRow, Type};
use serde::{Serialize, Deserialize};

// Mirror the device_status enum from the migration
#[derive(Debug, Clone, PartialEq, Eq, Type, Serialize, Deserialize)]
#[sqlx(type_name = "device_status", rename_all = "lowercase")]
pub enum DeviceStatus {
    Up,
    Down,
    Unknown,
}

// Allow converting from String (case-insensitive) for manual mapping
impl TryFrom<String> for DeviceStatus {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "up" => Ok(DeviceStatus::Up),
            "down" => Ok(DeviceStatus::Down),
            "unknown" => Ok(DeviceStatus::Unknown),
            _ => Err(format!("Invalid device status string: {}", value)),
        }
    }
}

// Struct corresponding to the 'devices' table
#[derive(Debug, Clone, PartialEq, FromRow, Serialize, Deserialize)]
pub struct Device {
    pub id: Uuid,
    pub hostname: Option<String>, // Made Option<> as UNIQUE allows one NULL
    pub ip_address: IpNetwork, // sqlx maps INET to ipnetwork::IpNetwork
    pub sys_name: Option<String>,
    pub sys_descr: Option<String>,
    pub vendor: Option<String>,
    pub model: Option<String>,
    pub os_version: Option<String>,
    pub serial_number: Option<String>,
    pub status: Option<DeviceStatus>, // Mapped from device_status enum
    pub last_seen: Option<OffsetDateTime>, // TIMESTAMPTZ maps to OffsetDateTime
    pub created_at: OffsetDateTime, 
    pub updated_at: OffsetDateTime,
}

// Struct corresponding to the 'interfaces' table
#[derive(Debug, Clone, PartialEq, FromRow, Serialize, Deserialize)]
pub struct Interface {
    pub id: Uuid,
    pub device_id: Uuid,
    pub if_index: i32, // INTEGER maps to i32
    pub if_name: Option<String>,
    pub if_alias: Option<String>,
    pub if_descr: Option<String>,
    pub if_type: Option<String>,
    // pub mac_address: Option<[u8; 6]>, // sqlx currently needs feature for MACADDR, handle differently or use String for now
    pub mac_address: Option<String>, // Using String for MAC for now
    pub ip_address: Option<IpNetwork>,
    pub admin_status: Option<String>,
    pub oper_status: Option<String>,
    pub speed: Option<i64>, // BIGINT maps to i64
    pub mtu: Option<i32>,
    pub last_changed: Option<OffsetDateTime>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
} 