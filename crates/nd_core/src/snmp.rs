// crates/nd_core/src/snmp.rs
use snmp::{SyncSession, Value, SnmpPdu, ObjectIdentifier};
use std::net::{IpAddr, ToSocketAddrs};
use std::time::Duration;
use tokio::task;

/// Format an SNMP OID (Vec<u32>) as a dot-separated string.
pub fn format_oid(oid: &[u32]) -> String {
    oid.iter().map(u32::to_string).collect::<Vec<_>>().join(".")
}

// Define our own owned version of snmp::Value
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SnmpValueOwned {
    Null,
    Integer(i64),
    OctetString(Vec<u8>),
    ObjectIdentifier(Vec<u32>),
    IpAddress([u8; 4]),
    Counter32(u32),
    Gauge32(u32),
    TimeTicks(u32),
    Opaque(Vec<u8>),
    Counter64(u64),
}

#[derive(Debug, thiserror::Error)]
pub enum SnmpError {
    #[error("SNMP communication error: {0}")]
    Snmp(String),
    #[error("IO error: {0}")]
    Io(String),
    #[error("Task join error: {0}")]
    Join(String),
    #[error("Response OID mismatch (expected {expected:?}, got {got:?})")]
    OidMismatch { expected: Vec<u32>, got: Vec<u32> },
    #[error("Response contained no variable bindings or null value")]
    NoVarBindValue,
    #[error("OID Processing error: {0}")]
    OidProcessing(String),
}

/// Converts an snmp::Value to SnmpValueOwned.
fn to_owned_value(value: Value) -> Result<SnmpValueOwned, SnmpError> {
    match value {
        Value::Null => Ok(SnmpValueOwned::Null),
        Value::Integer(i) => Ok(SnmpValueOwned::Integer(i)),
        Value::OctetString(s) => Ok(SnmpValueOwned::OctetString(s.to_vec())),
        Value::ObjectIdentifier(oid_ref) => {
            let mut oid_buf = [0u32; 128];
            let oid_parts = oid_ref.read_name(&mut oid_buf)
                .map_err(|e| SnmpError::OidProcessing(format!("Failed to read OID: {:?}", e)))?;
            Ok(SnmpValueOwned::ObjectIdentifier(oid_parts.to_vec()))
        },
        Value::IpAddress(ip) => Ok(SnmpValueOwned::IpAddress(ip)),
        Value::Counter32(c) => Ok(SnmpValueOwned::Counter32(c)),
        Value::Unsigned32(g) => Ok(SnmpValueOwned::Gauge32(g)),
        Value::Timeticks(t) => Ok(SnmpValueOwned::TimeTicks(t)),
        Value::Opaque(o) => Ok(SnmpValueOwned::Opaque(o.to_vec())),
        Value::Counter64(c) => Ok(SnmpValueOwned::Counter64(c)),
        _ => {
            tracing::warn!(value_type = ?value, "Unhandled SNMP value type encountered during conversion");
            Err(SnmpError::OidProcessing("Unhandled SNMP value type".to_string()))
        }
    }
}

/// Performs an SNMPv2c GET request for a single OID using spawn_blocking.
/// Returns an owned value.
pub async fn snmp_get_v2c(
    target_addr: &str,
    community: &[u8],
    oid: ObjectIdentifier<'static>,
) -> Result<SnmpValueOwned, SnmpError> {
    let target_owned = target_addr.to_string();
    let community_owned = community.to_vec();

    task::spawn_blocking(move || -> Result<SnmpValueOwned, SnmpError> {
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

        let mut oid_buf = [0u32; 128];
        let oid_slice = oid.read_name(&mut oid_buf)
            .map_err(|e| SnmpError::OidProcessing(format!("Failed to read OID: {:?}", e)))?;
        let response: SnmpPdu = sess.get(oid_slice)
            .map_err(|e| SnmpError::Snmp(format!("GET failed: {:?}", e)))?;

        let mut varbinds_iter = response.varbinds.into_iter();
        if let Some((response_oid_raw, value)) = varbinds_iter.next() {
            if varbinds_iter.next().is_some() {
                tracing::warn!(target = %target_owned, oid = ?oid, "Received multiple varbinds for single GET");
            }

            let mut expected_buf = [0u32; 128];
            let expected_oid_parts = oid.read_name(&mut expected_buf)
                .map_err(|e| SnmpError::OidProcessing(format!("Failed to read OID: {:?}", e)))?;

            let mut resp_buf = [0u32; 128];
            let response_oid_parts = response_oid_raw.read_name(&mut resp_buf)
                .map_err(|e| SnmpError::OidProcessing(format!("Failed to read OID: {:?}", e)))?;

            if response_oid_parts != expected_oid_parts {
                return Err(SnmpError::OidMismatch {
                    expected: expected_oid_parts.to_vec(),
                    got: response_oid_parts.to_vec(),
                });
            }

            to_owned_value(value)
        } else {
            Err(SnmpError::NoVarBindValue)
        }
    }).await.map_err(|e| SnmpError::Join(e.to_string()))?
}

/// Performs an SNMPv2c GETNEXT walk for a table OID using spawn_blocking.
/// Returns a vector of (OID, OwnedValue) pairs.
pub async fn snmp_walk_v2c(
    target_addr: IpAddr,
    community: &str,
    base_oid: &[u32],
) -> Result<Vec<(Vec<u32>, SnmpValueOwned)>, SnmpError> {
    let target_owned = target_addr.to_string();
    let community_owned = community.as_bytes().to_vec();
    let base_oid_owned = base_oid.to_vec();

    task::spawn_blocking(move || -> Result<Vec<(Vec<u32>, SnmpValueOwned)>, SnmpError> {
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
        let mut current_oid_vec = base_oid_owned.clone();

        loop {
            let pdu: SnmpPdu = match sess.getnext(current_oid_vec.as_slice()) {
                Ok(pdu) => pdu,
                Err(e) => {
                    tracing::error!(target = %target_owned, oid = ?current_oid_vec, "SNMP getnext error: {:?}", e);
                    return Err(SnmpError::Snmp(format!("GETNEXT failed: {:?}", e)));
                }
            };

            let mut varbinds_iter = pdu.varbinds.into_iter();
            let next_varbind = match varbinds_iter.next() {
                Some(vb) => vb,
                None => {
                    tracing::debug!(target = %target_owned, base_oid = ?base_oid_owned, "Walk finished: No more varbinds returned by agent");
                    break;
                }
            };

            if varbinds_iter.next().is_some() {
                tracing::warn!(target = %target_owned, base_oid = ?base_oid_owned, "Received multiple varbinds for single GETNEXT request");
            }

            let (next_oid_raw, value) = next_varbind;
            let mut oid_buf = [0u32; 128];
            let next_oid_parts = next_oid_raw.read_name(&mut oid_buf)
                .map_err(|e| SnmpError::OidProcessing(format!("Failed to read OID: {:?}", e)))?;

            if !next_oid_parts.starts_with(&base_oid_owned) {
                tracing::debug!(target = %target_owned, base_oid = ?base_oid_owned, next_oid = ?next_oid_parts, "Walk finished: OID left requested subtree");
                break;
            }

            if matches!(value, Value::Null) {
                tracing::trace!(target = %target_owned, next_oid = ?next_oid_parts, "Received Null value during walk.");
            }

            let owned_value = to_owned_value(value)?;
            results.push((next_oid_parts.to_vec(), owned_value));

            current_oid_vec = next_oid_parts.to_vec();
        }

        Ok(results)
    }).await.map_err(|e| SnmpError::Join(e.to_string()))?
}

#[cfg(test)]
mod tests {
    use super::*;
    use snmp::Value;
    use std::net::IpAddr;

    #[test]
    fn test_format_oid() {
        let oid = [1, 3, 6, 1, 2, 1];
        let formatted = format_oid(&oid);
        assert_eq!(formatted, "1.3.6.1.2.1");
    }

    #[test]
    fn test_to_owned_value_integer() {
        let value = Value::Integer(42);
        let owned = to_owned_value(value).unwrap();
        assert_eq!(owned, SnmpValueOwned::Integer(42));
    }

    #[tokio::test]
    #[ignore]
    async fn test_snmp_get_v2c_failure() {
        // Using an unreachable address, reserved for documentation, to force an error.
        let result = snmp_get_v2c("192.0.2.0", b"public", &[1, 3, 6, 1, 2, 1]).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    #[ignore]
    async fn test_snmp_walk_v2c_failure() {
        // Using an unreachable IP address to ensure the walk fails.
        let target: IpAddr = "192.0.2.0".parse().unwrap();
        let result = snmp_walk_v2c(target, "public", &[1, 3, 6, 1, 2, 1]).await;
        assert!(result.is_err());
    }
}