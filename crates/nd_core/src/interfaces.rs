use std::net::IpAddr;
use thiserror::Error;

use crate::snmp::{snmp_walk_v2c, SnmpError, SnmpValueOwned, format_oid};

// Revert OID constants to &[u32]
const IF_INDEX_OID: &[u32] = &[1, 3, 6, 1, 2, 1, 2, 2, 1, 1];
const IF_DESCR_OID: &[u32] = &[1, 3, 6, 1, 2, 1, 2, 2, 1, 2];
const IF_TYPE_OID: &[u32] = &[1, 3, 6, 1, 2, 1, 2, 2, 1, 3];
const IF_MTU_OID: &[u32] = &[1, 3, 6, 1, 2, 1, 2, 2, 1, 4];
const IF_SPEED_OID: &[u32] = &[1, 3, 6, 1, 2, 1, 2, 2, 1, 5];
const IF_PHYS_ADDR_OID: &[u32] = &[1, 3, 6, 1, 2, 1, 2, 2, 1, 6];
const IF_ADMIN_STATUS_OID: &[u32] = &[1, 3, 6, 1, 2, 1, 2, 2, 1, 7];
const IF_OPER_STATUS_OID: &[u32] = &[1, 3, 6, 1, 2, 1, 2, 2, 1, 8];
const IF_LAST_CHANGE_OID: &[u32] = &[1, 3, 6, 1, 2, 1, 2, 2, 1, 9];
const IF_IN_OCTETS_OID: &[u32] = &[1, 3, 6, 1, 2, 1, 2, 2, 1, 10];
const IF_OUT_OCTETS_OID: &[u32] = &[1, 3, 6, 1, 2, 2, 1, 16];

#[derive(Debug, Error)]
pub enum InterfaceCollectionError {
    #[error("SNMP processing error: {0}")]
    SnmpProcessing(#[from] SnmpError),
    
    #[error("No interfaces found")]
    NoInterfaces,
    
    #[error("Invalid data format: {0}")]
    InvalidData(String),
    
    #[error("OID parsing error: {0}")]
    OidParseError(String),
    
    #[error("SNMP walk failed for OID {oid}: {source}")]
    WalkFailed { oid: String, source: SnmpError },
}

/// Represents a network interface
#[derive(Debug, Clone)]
pub struct Interface {
    pub index: u32,
    pub description: String,
    pub interface_type: u32,
    pub mtu: Option<i64>,
    pub speed: Option<u32>,
    pub physical_address: Option<Vec<u8>>,
    pub admin_status: u32,
    pub oper_status: u32,
    pub last_change: Option<u32>,
    pub in_octets: Option<u64>,
    pub out_octets: Option<u64>,
}

impl Interface {
    fn new(index: u32) -> Self {
        Interface {
            index,
            description: String::new(),
            interface_type: 0,
            mtu: None,
            speed: None,
            physical_address: None,
            admin_status: 0,
            oper_status: 0,
            last_change: None,
            in_octets: None,
            out_octets: None,
        }
    }
    
    /// Format MAC address as a hex string with colons
    fn format_mac_address(bytes: &[u8]) -> String {
        bytes.iter()
            .map(|b| format!("{:02x}", b))
            .collect::<Vec<String>>()
            .join(":")
    }
}

/// Helper to extract the index from the end of an OID
fn get_index_from_oid(oid_str: &str) -> Result<u32, InterfaceCollectionError> {
    let parts: Vec<&str> = oid_str.split('.').collect();
    parts.last()
        .and_then(|s| s.parse::<u32>().ok())
        .ok_or_else(|| InterfaceCollectionError::InvalidData("Failed to extract index from OID".to_string()))
}

/// Collect interface information from a device using SNMP walks.
pub async fn collect_interfaces(target: IpAddr, community: &str) -> Result<Vec<Interface>, InterfaceCollectionError> {
    let mut interfaces = Vec::new();
    let mut interface_indices: Vec<(u32, u32)> = Vec::new();

    // First collect all interface indices
    let index_results = snmp_walk_v2c(target, community, IF_INDEX_OID)
        .await
        .map_err(|e| InterfaceCollectionError::WalkFailed { oid: format_oid(IF_INDEX_OID), source: e })?;
    if index_results.is_empty() {
        return Err(InterfaceCollectionError::NoInterfaces);
    }

    for (oid, value) in index_results {
        if let SnmpValueOwned::Integer(index) = value {
            interface_indices.push((get_index_from_oid(&format_oid(&oid))?, index as u32));
        }
    }

    // Now collect all interface descriptions
    let mut descriptions: Vec<(u32, String)> = Vec::new();
    let descr_results = snmp_walk_v2c(target, community, IF_DESCR_OID)
        .await
        .map_err(|e| InterfaceCollectionError::WalkFailed { oid: format_oid(IF_DESCR_OID), source: e })?;
    for (oid, value) in descr_results {
        if let SnmpValueOwned::OctetString(bytes) = value {
            descriptions.push((
                get_index_from_oid(&format_oid(&oid))?,
                String::from_utf8_lossy(&bytes).into()
            ));
        }
    }

    // Collect interface types
    let mut types: Vec<(u32, u32)> = Vec::new();
    let type_results = snmp_walk_v2c(target, community, IF_TYPE_OID)
        .await
        .map_err(|e| InterfaceCollectionError::WalkFailed { oid: format_oid(IF_TYPE_OID), source: e })?;
    for (oid, value) in type_results {
        if let SnmpValueOwned::Integer(if_type) = value {
            types.push((get_index_from_oid(&format_oid(&oid))?, if_type as u32));
        }
    }

    // Collect MTUs
    let mut mtus: Vec<(u32, Option<i64>)> = Vec::new();
    let mtu_results = snmp_walk_v2c(target, community, IF_MTU_OID)
        .await
        .map_err(|e| InterfaceCollectionError::WalkFailed { oid: format_oid(IF_MTU_OID), source: e })?;
    for (oid, value) in mtu_results {
        if let SnmpValueOwned::Integer(mtu) = value {
            mtus.push((get_index_from_oid(&format_oid(&oid))?, Some(mtu)));
        }
    }

    // Collect speeds
    let mut speeds: Vec<(u32, Option<u32>)> = Vec::new();
    let speed_results = snmp_walk_v2c(target, community, IF_SPEED_OID)
        .await
        .map_err(|e| InterfaceCollectionError::WalkFailed { oid: format_oid(IF_SPEED_OID), source: e })?;
    for (oid, value) in speed_results {
        if let SnmpValueOwned::Gauge32(speed) = value {
            speeds.push((get_index_from_oid(&format_oid(&oid))?, Some(speed)));
        }
    }

    // Collect physical addresses
    let mut phys_addrs: Vec<(u32, Option<Vec<u8>>)> = Vec::new();
    let phys_addr_results = snmp_walk_v2c(target, community, IF_PHYS_ADDR_OID)
        .await
        .map_err(|e| InterfaceCollectionError::WalkFailed { oid: format_oid(IF_PHYS_ADDR_OID), source: e })?;
    for (oid, value) in phys_addr_results {
        if let SnmpValueOwned::OctetString(addr) = value {
            phys_addrs.push((get_index_from_oid(&format_oid(&oid))?, Some(addr)));
        }
    }

    // Collect admin statuses
    let mut admin_statuses: Vec<(u32, u32)> = Vec::new();
    let admin_status_results = snmp_walk_v2c(target, community, IF_ADMIN_STATUS_OID)
        .await
        .map_err(|e| InterfaceCollectionError::WalkFailed { oid: format_oid(IF_ADMIN_STATUS_OID), source: e })?;
    for (oid, value) in admin_status_results {
        if let SnmpValueOwned::Integer(status) = value {
            admin_statuses.push((get_index_from_oid(&format_oid(&oid))?, status as u32));
        }
    }

    // Collect operational statuses
    let mut oper_statuses: Vec<(u32, u32)> = Vec::new();
    let oper_status_results = snmp_walk_v2c(target, community, IF_OPER_STATUS_OID)
        .await
        .map_err(|e| InterfaceCollectionError::WalkFailed { oid: format_oid(IF_OPER_STATUS_OID), source: e })?;
    for (oid, value) in oper_status_results {
        if let SnmpValueOwned::Integer(status) = value {
            oper_statuses.push((get_index_from_oid(&format_oid(&oid))?, status as u32));
        }
    }

    // Collect last change times
    let mut last_changes: Vec<(u32, Option<u32>)> = Vec::new();
    let last_change_results = snmp_walk_v2c(target, community, IF_LAST_CHANGE_OID)
        .await
        .map_err(|e| InterfaceCollectionError::WalkFailed { oid: format_oid(IF_LAST_CHANGE_OID), source: e })?;
    for (oid, value) in last_change_results {
        if let SnmpValueOwned::TimeTicks(ticks) = value {
            last_changes.push((get_index_from_oid(&format_oid(&oid))?, Some(ticks)));
        }
    }

    // Collect input octets
    let mut in_octets: Vec<(u32, Option<u64>)> = Vec::new();
    let in_octets_results = snmp_walk_v2c(target, community, IF_IN_OCTETS_OID)
        .await
        .map_err(|e| InterfaceCollectionError::WalkFailed { oid: format_oid(IF_IN_OCTETS_OID), source: e })?;
    for (oid, value) in in_octets_results {
        match value {
            SnmpValueOwned::Counter32(count) => {
                in_octets.push((get_index_from_oid(&format_oid(&oid))?, Some(count as u64)));
            }
            SnmpValueOwned::Counter64(count) => {
                in_octets.push((get_index_from_oid(&format_oid(&oid))?, Some(count)));
            }
            _ => {}
        }
    }

    // Collect output octets
    let mut out_octets: Vec<(u32, Option<u64>)> = Vec::new();
    let out_octets_results = snmp_walk_v2c(target, community, IF_OUT_OCTETS_OID)
        .await
        .map_err(|e| InterfaceCollectionError::WalkFailed { oid: format_oid(IF_OUT_OCTETS_OID), source: e })?;
    for (oid, value) in out_octets_results {
        match value {
            SnmpValueOwned::Counter32(count) => {
                out_octets.push((get_index_from_oid(&format_oid(&oid))?, Some(count as u64)));
            }
            SnmpValueOwned::Counter64(count) => {
                out_octets.push((get_index_from_oid(&format_oid(&oid))?, Some(count)));
            }
            _ => {}
        }
    }

    // Build interface objects
    for (idx, index) in interface_indices {
        let description = descriptions.iter()
            .find(|(i, _)| *i == idx)
            .map(|(_, d)| d.clone())
            .unwrap_or_default();

        let interface_type = types.iter()
            .find(|(i, _)| *i == idx)
            .map(|(_, t)| *t)
            .unwrap_or_default();

        let mtu = mtus.iter()
            .find(|(i, _)| *i == idx)
            .and_then(|(_, m)| *m);

        let speed = speeds.iter()
            .find(|(i, _)| *i == idx)
            .and_then(|(_, s)| *s);

        let physical_address = phys_addrs.iter()
            .find(|(i, _)| *i == idx)
            .and_then(|(_, a)| a.clone());

        let admin_status = admin_statuses.iter()
            .find(|(i, _)| *i == idx)
            .map(|(_, s)| *s)
            .unwrap_or_default();

        let oper_status = oper_statuses.iter()
            .find(|(i, _)| *i == idx)
            .map(|(_, s)| *s)
            .unwrap_or_default();

        let last_change = last_changes.iter()
            .find(|(i, _)| *i == idx)
            .and_then(|(_, t)| *t);

        let in_octets_val = in_octets.iter()
            .find(|(i, _)| *i == idx)
            .and_then(|(_, o)| *o);

        let out_octets_val = out_octets.iter()
            .find(|(i, _)| *i == idx)
            .and_then(|(_, o)| *o);

        interfaces.push(Interface {
            index,
            description,
            interface_type,
            mtu,
            speed,
            physical_address,
            admin_status,
            oper_status,
            last_change,
            in_octets: in_octets_val,
            out_octets: out_octets_val,
        });
    }

    Ok(interfaces)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::IpAddr;
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};

    // --- Mock snmp_walk_v2c ---
    type MockWalkResults = Arc<Mutex<HashMap<String, Result<Vec<(Vec<u32>, SnmpValueOwned)>, SnmpError>>>>;

    tokio::task_local! {
        static MOCK_RESULTS: MockWalkResults;
    }

    async fn snmp_walk_v2c(
        _target_addr: IpAddr,
        _community: &str,
        base_oid: &[u32],
    ) -> Result<Vec<(Vec<u32>, SnmpValueOwned)>, SnmpError> {
        MOCK_RESULTS.with(|results| {
            let mut locked_results = results.lock().unwrap();
            let base_oid_str = format_oid(base_oid);

            locked_results.remove(&base_oid_str)
                .unwrap_or_else(|| {
                    println!("Mock Warning: No predefined result for OID {}, returning empty vec.", base_oid_str);
                    Ok(Vec::new())
                })
        })
    }

    // Helper to set up mock results for a test
    fn setup_mock_results(results_map: HashMap<String, Result<Vec<(Vec<u32>, SnmpValueOwned)>, SnmpError>>) -> MockWalkResults {
        Arc::new(Mutex::new(results_map))
    }

    // Helper to create an OID for test data
    fn oid_for_test(base_oid: &[u32], index: u32) -> Vec<u32> {
        let mut oid_vec = base_oid.to_vec();
        oid_vec.push(index);
        oid_vec
    }

    #[tokio::test]
    async fn test_collect_interfaces_success() {
        let mut mock_data = HashMap::new();

        mock_data.insert(
            format_oid(IF_INDEX_OID),
            Ok(vec![
                (oid_for_test(IF_INDEX_OID, 1), SnmpValueOwned::Integer(1)),
                (oid_for_test(IF_INDEX_OID, 2), SnmpValueOwned::Integer(2)),
            ])
        );

        mock_data.insert(
            format_oid(IF_DESCR_OID),
            Ok(vec![
                (oid_for_test(IF_DESCR_OID, 1), SnmpValueOwned::OctetString(b"Ethernet Interface".to_vec())),
                (oid_for_test(IF_DESCR_OID, 2), SnmpValueOwned::OctetString(b"Loopback".to_vec())),
            ])
        );

        let other_oids = [
            IF_TYPE_OID, IF_MTU_OID, IF_SPEED_OID, IF_PHYS_ADDR_OID,
            IF_ADMIN_STATUS_OID, IF_OPER_STATUS_OID
        ];
        for oid in other_oids {
            mock_data.insert(format_oid(oid), Ok(Vec::new()));
        }

        let mock_state = setup_mock_results(mock_data);

        let target_ip: IpAddr = "127.0.0.1".parse().unwrap();
        let result = MOCK_RESULTS.scope(mock_state, async {
            collect_interfaces(target_ip, "public").await
        }).await;

        assert!(result.is_ok(), "collect_interfaces failed: {:?}", result.err());
        let interfaces = result.unwrap();
        assert_eq!(interfaces.len(), 2);

        let if1 = interfaces.iter().find(|i| i.index == 1).expect("Interface 1 not found");
        assert_eq!(if1.description, "Ethernet Interface");

        let if2 = interfaces.iter().find(|i| i.index == 2).expect("Interface 2 not found");
        assert_eq!(if2.description, "Loopback");

        assert!(if1.interface_type == 0);
        assert!(if1.physical_address.is_none());
        assert!(if2.interface_type == 0);
    }

    #[tokio::test]
    async fn test_collect_interfaces_walk_error() {
        let mut mock_data = HashMap::new();
        mock_data.insert(
            format_oid(IF_INDEX_OID),
            Ok(vec![
                (oid_for_test(IF_INDEX_OID, 1), SnmpValueOwned::Integer(1)),
            ])
        );
        mock_data.insert(
            format_oid(IF_DESCR_OID),
            Err(SnmpError::Snmp("Simulated walk error".to_string()))
        );

        let other_oids = [
            IF_TYPE_OID, IF_MTU_OID, IF_SPEED_OID, IF_PHYS_ADDR_OID,
            IF_ADMIN_STATUS_OID, IF_OPER_STATUS_OID
        ];
        for oid in other_oids {
            mock_data.insert(format_oid(oid), Ok(Vec::new()));
        }

        let mock_state = setup_mock_results(mock_data);

        let target_ip: IpAddr = "127.0.0.1".parse().unwrap();
        let result = MOCK_RESULTS.scope(mock_state, async {
            collect_interfaces(target_ip, "public").await
        }).await;

        assert!(result.is_err());
        match result.err().unwrap() {
            InterfaceCollectionError::WalkFailed { oid, source } => {
                assert_eq!(oid, format_oid(IF_DESCR_OID));
                assert!(matches!(source, SnmpError::Snmp(_)));
            },
            e => panic!("Expected WalkFailed error, got {:?}", e),
        }
    }

    #[tokio::test]
    async fn test_collect_interfaces_no_interfaces_found() {
        let mut mock_data = HashMap::new();

        let all_oids = [
            IF_INDEX_OID, IF_DESCR_OID, IF_TYPE_OID, IF_MTU_OID, IF_SPEED_OID,
            IF_PHYS_ADDR_OID, IF_ADMIN_STATUS_OID, IF_OPER_STATUS_OID
        ];
        for oid in all_oids {
            mock_data.insert(format_oid(oid), Ok(Vec::new()));
        }

        let mock_state = setup_mock_results(mock_data);

        let target_ip: IpAddr = "127.0.0.1".parse().unwrap();
        let result = MOCK_RESULTS.scope(mock_state, async {
            collect_interfaces(target_ip, "public").await
        }).await;

        assert!(result.is_err());
        assert!(matches!(result.err().unwrap(), InterfaceCollectionError::NoInterfaces));
    }
} 