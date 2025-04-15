use std::net::IpAddr;
use oid::ObjectIdentifier;
use thiserror::Error;
use std::collections::HashMap;

use crate::{snmp_walk_v2c, SnmpError, SnmpValueOwned};

// Standard IF-MIB OIDs
const IF_INDEX_OID: &str = "1.3.6.1.2.1.2.2.1.1";
const IF_DESCR_OID: &str = "1.3.6.1.2.1.2.2.1.2";
const IF_TYPE_OID: &str = "1.3.6.1.2.1.2.2.1.3";
const IF_MTU_OID: &str = "1.3.6.1.2.1.2.2.1.4";
const IF_SPEED_OID: &str = "1.3.6.1.2.1.2.2.1.5";
const IF_PHYS_ADDRESS_OID: &str = "1.3.6.1.2.1.2.2.1.6";
const IF_ADMIN_STATUS_OID: &str = "1.3.6.1.2.1.2.2.1.7";
const IF_OPER_STATUS_OID: &str = "1.3.6.1.2.1.2.2.1.8";
const IF_NAME_OID: &str = "1.3.6.1.2.1.31.1.1.1.1";
const IF_ALIAS_OID: &str = "1.3.6.1.2.1.31.1.1.1.18";

#[derive(Debug, Error)]
pub enum InterfaceCollectionError {
    #[error("SNMP error: {0}")]
    Snmp(#[from] SnmpError),
    
    #[error("No interfaces found")]
    NoInterfaces,
    
    #[error("Invalid data format: {0}")]
    InvalidData(String),
    
    #[error("OID parsing error: {0}")]
    OidParse(#[from] oid::Error),
    
    #[error("SNMP walk failed for OID {oid}: {source}")]
    WalkFailed { oid: String, source: SnmpError },
}

/// Represents a network interface
#[derive(Debug, Clone)]
pub struct Interface {
    pub if_index: i32,
    pub if_name: Option<String>,
    pub if_descr: Option<String>,
    pub if_type: Option<i32>,
    pub if_mtu: Option<i32>,
    pub if_speed: Option<u64>,
    pub mac_address: Option<String>,
    pub admin_status: Option<i32>,
    pub oper_status: Option<i32>,
    pub alias: Option<String>,
}

impl Interface {
    fn new(if_index: i32) -> Self {
        Interface {
            if_index,
            if_name: None,
            if_descr: None,
            if_type: None,
            if_mtu: None,
            if_speed: None,
            mac_address: None,
            admin_status: None,
            oper_status: None,
            alias: None,
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
fn get_index_from_oid(oid: &ObjectIdentifier) -> Option<i32> {
    oid.iter().last().and_then(|idx_u32| (*idx_u32).try_into().ok())
}

/// Collect interface information from a device using SNMP walks.
pub async fn collect_interfaces(
    ip_address: IpAddr,
    community: &str,
) -> Result<Vec<Interface>, InterfaceCollectionError> {
    let mut interfaces_map: HashMap<i32, Interface> = HashMap::new();

    // --- Define OIDs to walk ---
    let oids_to_walk = [
        IF_INDEX_OID, 
        IF_DESCR_OID, 
        IF_TYPE_OID, 
        IF_MTU_OID,
        IF_SPEED_OID,
        IF_PHYS_ADDRESS_OID,
        IF_ADMIN_STATUS_OID,
        IF_OPER_STATUS_OID,
        IF_NAME_OID,
        IF_ALIAS_OID,
    ];

    // --- Perform SNMP Walks Concurrently (Optional Optimization) ---
    // For simplicity, we'll walk sequentially here. Tokio::join! could parallelize.
    for oid_str in oids_to_walk {
        let base_oid = ObjectIdentifier::parse(oid_str)?;
        let walk_results = snmp_walk_v2c(ip_address, community, &base_oid)
            .await
            .map_err(|e| InterfaceCollectionError::WalkFailed { oid: oid_str.to_string(), source: e })?;

        // --- Process Walk Results ---    
        for (oid, value) in walk_results {
            if let Some(if_index) = get_index_from_oid(&oid) {
                let entry = interfaces_map.entry(if_index).or_insert_with(|| Interface::new(if_index));

                match oid_str { // Match on the *base* OID string we walked
                    IF_INDEX_OID => { 
                        // We use IF_INDEX just to discover indices initially, value not stored directly
                        // This branch might be removable if IF_NAME or IF_DESCR is guaranteed
                    },
                    IF_DESCR_OID => {
                        if let SnmpValueOwned::OctetString(bytes) = value {
                            entry.if_descr = Some(String::from_utf8_lossy(&bytes).to_string());
                        }
                    },
                    IF_TYPE_OID => {
                        if let SnmpValueOwned::Integer(val) = value {
                            entry.if_type = Some(val as i32);
                        }
                    },
                    IF_MTU_OID => {
                        if let SnmpValueOwned::Integer(val) = value {
                            entry.if_mtu = Some(val as i32);
                        }
                    },
                    IF_SPEED_OID => {
                        match value {
                            SnmpValueOwned::Gauge32(val) => entry.if_speed = Some(val as u64),
                            SnmpValueOwned::Integer(val) => entry.if_speed = Some(val as u64), // Less common, but possible
                            SnmpValueOwned::Counter32(val) => entry.if_speed = Some(val as u64), // Sometimes used
                            _ => { /* Log warning or ignore */ }
                        }
                    },
                    IF_PHYS_ADDRESS_OID => {
                         if let SnmpValueOwned::OctetString(bytes) = value {
                             if !bytes.is_empty() {
                                 entry.mac_address = Some(Interface::format_mac_address(&bytes));
                             }
                         }
                    },
                     IF_ADMIN_STATUS_OID => {
                         if let SnmpValueOwned::Integer(val) = value {
                             entry.admin_status = Some(val as i32);
                         }
                     },
                     IF_OPER_STATUS_OID => {
                         if let SnmpValueOwned::Integer(val) = value {
                             entry.oper_status = Some(val as i32);
                         }
                     },
                     IF_NAME_OID => {
                         if let SnmpValueOwned::OctetString(bytes) = value {
                             entry.if_name = Some(String::from_utf8_lossy(&bytes).to_string());
                         }
                     },
                     IF_ALIAS_OID => {
                         if let SnmpValueOwned::OctetString(bytes) = value {
                              let alias_str = String::from_utf8_lossy(&bytes).to_string();
                              if !alias_str.is_empty() {
                                 entry.alias = Some(alias_str);
                              }
                         }
                     },
                    _ => { /* Unknown OID - should not happen based on loop */ }
                }
            }
        }
    }

    // --- Final Processing --- 
    let interfaces: Vec<Interface> = interfaces_map.into_values().collect();

    if interfaces.is_empty() {
        // Check if walks returned data but no valid indices were found, 
        // or if the walks themselves truly returned nothing.
        // This might require checking the raw walk_results before processing.
        tracing::warn!(target = %ip_address, "No interfaces collected after SNMP walks.");
        // We might still return Ok(vec![]) if the device genuinely has no interfaces responding
        // Or return NoInterfaces error if appropriate based on walk results.
         Err(InterfaceCollectionError::NoInterfaces) // Keep original behavior for now
    } else {
        Ok(interfaces)
    }
} 