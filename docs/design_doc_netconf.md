# NETCONF Support Design Document

## Overview
This document outlines the design and implementation of NETCONF support in NetDisco-rust. NETCONF (Network Configuration Protocol) is a protocol defined in RFC 6241 that provides mechanisms to install, manipulate, and delete the configuration of network devices.

## Components

### 1. NETCONF Client
- **Connection Management**: Handles SSH transport layer and NETCONF session establishment
- **RPC Operations**: Implements standard NETCONF operations (get, get-config, edit-config, etc.)
- **Message Formatting**: XML encoding/decoding for NETCONF messages
- **Capability Negotiation**: Handles NETCONF capabilities exchange

### 2. Data Models
- **YANG Model Support**: Parser and validator for YANG models
- **Schema Registry**: Management of device-specific YANG models
- **Data Transformation**: Conversion between YANG and internal data structures

### 3. Operations Manager
- **Operation Handlers**: Implementation of NETCONF operations
- **Transaction Management**: Handles configuration transactions
- **Validation**: Ensures operations comply with device capabilities
- **Error Handling**: Standardized error reporting and recovery

### 4. Device Integration
- **Device Discovery**: Detection of NETCONF-capable devices
- **Capability Management**: Tracking of device-supported operations
- **Authentication**: SSH key and password-based authentication
- **Session Management**: Connection pooling and session maintenance

## Database Schema

```sql
-- NETCONF Capabilities
CREATE TABLE netconf_capabilities (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    device_id UUID NOT NULL REFERENCES devices(id),
    capability TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (device_id, capability)
);

-- NETCONF Sessions
CREATE TABLE netconf_sessions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    device_id UUID NOT NULL REFERENCES devices(id),
    session_id TEXT NOT NULL,
    status TEXT NOT NULL,
    started_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_active_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    ended_at TIMESTAMPTZ,
    error_message TEXT
);

-- YANG Models
CREATE TABLE yang_models (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL,
    version TEXT NOT NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (name, version)
);

-- Device YANG Models
CREATE TABLE device_yang_models (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    device_id UUID NOT NULL REFERENCES devices(id),
    yang_model_id UUID NOT NULL REFERENCES yang_models(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (device_id, yang_model_id)
);
```

## API Endpoints

### NETCONF Operations
- `POST /api/v1/netconf/{device_id}/get` - Retrieve running configuration
- `POST /api/v1/netconf/{device_id}/get-config` - Retrieve specific configuration
- `POST /api/v1/netconf/{device_id}/edit-config` - Modify configuration
- `POST /api/v1/netconf/{device_id}/copy-config` - Copy configuration
- `POST /api/v1/netconf/{device_id}/delete-config` - Delete configuration
- `POST /api/v1/netconf/{device_id}/lock` - Lock configuration
- `POST /api/v1/netconf/{device_id}/unlock` - Unlock configuration
- `GET /api/v1/netconf/{device_id}/capabilities` - List device capabilities

### YANG Model Management
- `GET /api/v1/yang/models` - List available YANG models
- `POST /api/v1/yang/models` - Upload new YANG model
- `GET /api/v1/yang/models/{id}` - Get specific YANG model
- `DELETE /api/v1/yang/models/{id}` - Delete YANG model
- `GET /api/v1/devices/{device_id}/yang-models` - List device YANG models

## Implementation Phases

### Phase 1: Core NETCONF Client
1. Implement basic SSH transport layer
2. Add NETCONF session establishment
3. Implement basic RPC operations
4. Add XML encoding/decoding support

### Phase 2: YANG Support
1. Implement YANG model parser
2. Create schema registry
3. Add validation system
4. Implement data transformation

### Phase 3: Advanced Features
1. Add transaction management
2. Implement session pooling
3. Add advanced error handling
4. Create monitoring system

### Phase 4: Integration
1. Integrate with device manager
2. Add configuration system support
3. Implement backup system integration
4. Create API endpoints

## Error Handling
- Connection failures
- Authentication errors
- RPC operation failures
- Validation errors
- Transaction rollbacks
- Session timeouts

## Security Considerations
- SSH key management
- Password security
- Session isolation
- Access control
- Audit logging
- Secure storage of credentials

## Testing Strategy
1. Unit tests for individual components
2. Integration tests with test devices
3. Capability negotiation tests
4. Error handling scenarios
5. Performance testing
6. Security testing

## Dependencies
- `netconf` - NETCONF protocol implementation
- `ssh2` - SSH transport layer
- `quick-xml` - XML processing
- `yang2` - YANG model parsing
- `tokio` - Async runtime
- `tracing` - Logging and diagnostics 