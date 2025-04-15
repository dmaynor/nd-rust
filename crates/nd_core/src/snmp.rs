// crates/nd_core/src/snmp.rs
use snmp::{SyncSession, Value};
use std::net::ToSocketAddrs;
use std::time::Duration;
use tokio::task;

// Define our own owned version of snmp::Value
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SnmpValueOwned {
    Null, // Explicitly handle Null
    Integer(i64),
    OctetString(Vec<u8>),
    ObjectIdentifier(Vec<u32>),
    IpAddress([u8; 4]),
    Counter32(u32),
    Gauge32(u32),
    TimeTicks(u32),
    Opaque(Vec<u8>),
    Counter64(u64),
    // No error variants here
}

#[derive(Debug, thiserror::Error)]
pub enum SnmpError {
    #[error("SNMP communication error: {0}")]
    Snmp(String),
    #[error("IO error: {0}")]
    Io(String), // Store IO error as string
    #[error("Task join error: {0}")]
    Join(String), // Store JoinError as string
    #[error("Response OID mismatch (expected {expected:?}, got {got:?})")]
    OidMismatch { expected: Vec<u32>, got: Vec<u32> },
    #[error("Response contained no variable bindings or null value")]
    NoVarBindValue,
}

/// Performs an SNMPv2c GET request for a single OID using spawn_blocking.
/// Returns an owned value.
pub async fn snmp_get_v2c(
    target_addr: &str,
    community: &[u8],
    oid_parts: &[u32],
) -> Result<SnmpValueOwned, SnmpError> {
    let target_owned = target_addr.to_string();
    let community_owned = community.to_vec();
    let oid_owned = oid_parts.to_vec();

    // Use map_err to convert potential errors within spawn_blocking
    task::spawn_blocking(move || -> Result<SnmpValueOwned, SnmpError> { // Add Result return type hint
        let socket_addr = (target_owned.as_str(), 161u16)
            .to_socket_addrs().map_err(|e| SnmpError::Io(e.to_string()))? // Map IO error
            .next()
            .ok_or_else(|| SnmpError::Io("Could not resolve target address".to_string()))?;

        let mut sess = SyncSession::new(
            socket_addr,
            &community_owned,
            Some(Duration::from_secs(5)),
            0,
        ).map_err(|e| SnmpError::Snmp(format!("{:?}", e)))?;

        let response = sess.get(&oid_owned).map_err(|e| SnmpError::Snmp(format!("{:?}", e)))?;

        let mut varbinds_iter = response.varbinds.into_iter();
        if let Some((response_oid, value)) = varbinds_iter.next() {
            if varbinds_iter.next().is_some() {
                tracing::warn!(target = %target_owned, oid = ?oid_owned, "Received multiple varbinds for single GET");
            }

            // Use a buffer to read the OID parts
            let mut oid_buf = [0u32; 128]; // ObjectIdentifier::read_name needs a buffer
            let response_oid_parts = response_oid.read_name(&mut oid_buf)
                .map_err(|e| SnmpError::Snmp(format!("Failed to read OID: {:?}", e)))?;
            
            let response_oid_vec = response_oid_parts.to_vec();
            if response_oid_vec != oid_owned.as_slice() {
                return Err(SnmpError::OidMismatch {
                    expected: oid_owned,
                    got: response_oid_vec,
                });
            }

            match value {
                Value::Null => Err(SnmpError::NoVarBindValue),
                Value::Integer(i) => Ok(SnmpValueOwned::Integer(i)),
                Value::OctetString(s) => Ok(SnmpValueOwned::OctetString(s.to_vec())),
                Value::ObjectIdentifier(oid) => {
                    let mut oid_buf = [0u32; 128];
                    let oid_parts = oid.read_name(&mut oid_buf)
                        .map_err(|e| SnmpError::Snmp(format!("Failed to read OID: {:?}", e)))?;
                    Ok(SnmpValueOwned::ObjectIdentifier(oid_parts.to_vec()))
                },
                Value::IpAddress(ip) => Ok(SnmpValueOwned::IpAddress(ip)),
                Value::Counter32(c) => Ok(SnmpValueOwned::Counter32(c)),
                // Handle Gauge32/Unsigned32
                Value::Unsigned32(g) => Ok(SnmpValueOwned::Gauge32(g)),
                Value::Timeticks(t) => Ok(SnmpValueOwned::TimeTicks(t)),
                Value::Opaque(o) => Ok(SnmpValueOwned::Opaque(o.to_vec())),
                Value::Counter64(c) => Ok(SnmpValueOwned::Counter64(c)),
                _ => {
                    // For any unhandled variants
                    tracing::warn!("Unhandled SNMP value type");
                    Err(SnmpError::NoVarBindValue)
                }
            }
        } else {
            Err(SnmpError::NoVarBindValue)
        }
    }).await.map_err(|e| SnmpError::Join(e.to_string()))? // Map JoinError and flatten Result<Result<_,_>,_>
} 