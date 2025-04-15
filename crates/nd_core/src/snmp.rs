// crates/nd_core/src/snmp.rs
use snmp::{SyncSession, Value};
use std::net::{IpAddr, ToSocketAddrs};
use std::time::Duration;
use tokio::task;
use oid::ObjectIdentifier;

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

/// Converts an snmp::Value to SnmpValueOwned.
fn to_owned_value(value: Value) -> Result<SnmpValueOwned, SnmpError> {
    match value {
        Value::Null => Ok(SnmpValueOwned::Null), // Return Null variant
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
        Value::Unsigned32(g) => Ok(SnmpValueOwned::Gauge32(g)),
        Value::Timeticks(t) => Ok(SnmpValueOwned::TimeTicks(t)),
        Value::Opaque(o) => Ok(SnmpValueOwned::Opaque(o.to_vec())),
        Value::Counter64(c) => Ok(SnmpValueOwned::Counter64(c)),
        _ => {
            tracing::warn!("Unhandled SNMP value type encountered during conversion");
            Err(SnmpError::Snmp("Unhandled SNMP value type".to_string()))
        }
    }
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

/// Performs an SNMPv2c GETNEXT walk for a table OID using spawn_blocking.
/// Returns a vector of (OID, OwnedValue) pairs.
pub async fn snmp_walk_v2c(
    target_addr: IpAddr, // Use IpAddr directly
    community: &str,     // Use &str
    base_oid: &ObjectIdentifier,
) -> Result<Vec<(ObjectIdentifier, SnmpValueOwned)>, SnmpError> {
    let target_owned = target_addr.to_string();
    let community_owned = community.as_bytes().to_vec(); // Convert &str to Vec<u8>
    let base_oid_owned = base_oid.clone(); // Clone base OID

    task::spawn_blocking(move || -> Result<Vec<(ObjectIdentifier, SnmpValueOwned)>, SnmpError> {
        let socket_addr = (target_owned.as_str(), 161u16)
            .to_socket_addrs().map_err(|e| SnmpError::Io(e.to_string()))?
            .next()
            .ok_or_else(|| SnmpError::Io("Could not resolve target address".to_string()))?;

        let mut sess = SyncSession::new(
            socket_addr,
            &community_owned,
            Some(Duration::from_secs(5)),
            0,
        ).map_err(|e| SnmpError::Snmp(format!("Session creation failed: {:?}", e)))?;

        let mut results = Vec::new();
        let mut current_oid = base_oid_owned.clone();

        loop {
            let response = match sess.getnext(&[current_oid.clone()]) {
                Ok(resp) => resp,
                Err(e) => {
                    // Distinguish between timeout and other errors if possible
                    // The snmp crate might not provide detailed error types here
                     tracing::error!(target = %target_owned, oid = %current_oid, "SNMP getnext error: {:?}", e);
                    return Err(SnmpError::Snmp(format!("GETNEXT failed: {:?}", e)));
                }
            };

            if response.varbinds.is_empty() {
                tracing::debug!(target = %target_owned, base_oid = %base_oid_owned, "Walk finished: No more varbinds");
                break; // No more results
            }

            let (next_oid, value) = response.varbinds.into_iter().next().unwrap(); // Only one varbind expected

            // Check if the returned OID is still within the requested subtree
             let mut oid_buf = [0u32; 128]; // Buffer needed
             let next_oid_parts = match next_oid.read_name(&mut oid_buf) {
                 Ok(parts) => parts,
                 Err(e) => return Err(SnmpError::Snmp(format!("Failed to read next OID: {:?}", e))),
             };
            let next_oid_object = ObjectIdentifier::from_slice(next_oid_parts);


             if !next_oid_object.starts_with(&base_oid_owned) {
                 tracing::debug!(target = %target_owned, base_oid = %base_oid_owned, next_oid = %next_oid_object, "Walk finished: OID left subtree");
                 break; // OID is outside the requested subtree
             }

            // Check for EndOfMibView
            if matches!(value, Value::EndOfMibView) {
                 tracing::debug!(target = %target_owned, base_oid = %base_oid_owned, next_oid = %next_oid_object, "Walk finished: EndOfMibView received");
                break;
            }

             let owned_value = to_owned_value(value)?;
             results.push((next_oid_object.clone(), owned_value));

            // Prepare for the next iteration
            current_oid = next_oid_object;
        }

        Ok(results)
    }).await.map_err(|e| SnmpError::Join(e.to_string()))? // Map JoinError
} 