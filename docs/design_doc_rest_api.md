# REST API Device Support Design Document

## Overview
The REST API Device Support feature enables NetDisco-rust to interact with modern network devices that expose REST APIs. This system will provide a unified interface for managing devices through their native REST APIs, supporting various vendor-specific implementations while maintaining a consistent internal API structure.

## Architecture

### Components

1. **REST Client Manager**
   - Handles HTTP/HTTPS connections to devices
   - Manages authentication and session tokens
   - Implements rate limiting and request queuing
   - Handles connection pooling and timeouts

2. **API Adapters**
   - Vendor-specific API implementations
   - Protocol translation layer
   - Response parsing and normalization
   - Error handling and recovery

3. **Device Discovery**
   - REST API endpoint discovery
   - Capability detection
   - API version identification
   - Authentication method detection

4. **Configuration Manager**
   - API endpoint configuration
   - Authentication credentials management
   - Rate limit settings
   - Retry policies

## Database Schema

```sql
-- REST API device configurations
CREATE TABLE rest_api_configs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    device_id UUID REFERENCES devices(id),
    api_type VARCHAR(50) NOT NULL,
    base_url VARCHAR(255) NOT NULL,
    api_version VARCHAR(50),
    auth_type VARCHAR(50) NOT NULL,
    auth_data JSONB,
    rate_limit INTEGER,
    timeout_ms INTEGER,
    verify_ssl BOOLEAN DEFAULT true,
    custom_headers JSONB,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- REST API capabilities
CREATE TABLE rest_api_capabilities (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    device_id UUID REFERENCES devices(id),
    capability_type VARCHAR(50) NOT NULL,
    endpoint_path VARCHAR(255) NOT NULL,
    http_method VARCHAR(10) NOT NULL,
    parameters JSONB,
    response_format VARCHAR(50),
    requires_auth BOOLEAN DEFAULT true,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- REST API session tokens
CREATE TABLE rest_api_sessions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    device_id UUID REFERENCES devices(id),
    token_type VARCHAR(50) NOT NULL,
    access_token TEXT NOT NULL,
    refresh_token TEXT,
    expires_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
```

## API Endpoints

### Device REST Configuration
- `GET /api/devices/{id}/rest-config` - Get device REST API configuration
- `POST /api/devices/{id}/rest-config` - Create/update REST API configuration
- `DELETE /api/devices/{id}/rest-config` - Remove REST API configuration

### Capability Management
- `GET /api/devices/{id}/capabilities` - List device REST API capabilities
- `POST /api/devices/{id}/capabilities/discover` - Trigger capability discovery
- `PUT /api/devices/{id}/capabilities/{capability_id}` - Update capability details

### Session Management
- `POST /api/devices/{id}/sessions` - Create new API session
- `GET /api/devices/{id}/sessions/current` - Get current session status
- `DELETE /api/devices/{id}/sessions/current` - Terminate current session

## Supported Vendors and APIs

### Cisco
- IOS-XE REST API
- NX-OS API
- DNA Center API
- Meraki Dashboard API

### Juniper
- Junos REST API
- Network Director API
- Mist API

### Arista
- eAPI
- CloudVision API

### HPE/Aruba
- ArubaOS-Switch REST API
- Aruba Central API
- HPE IMC API

## Implementation Phases

### Phase 1: Core Framework
1. Implement REST client manager
2. Create database schema
3. Develop basic API endpoints
4. Add authentication handling

### Phase 2: Vendor Support
1. Implement Cisco IOS-XE support
2. Add Juniper Junos support
3. Integrate Arista eAPI
4. Support Aruba REST API

### Phase 3: Advanced Features
1. Add capability discovery
2. Implement session management
3. Add rate limiting
4. Create monitoring system

## Error Handling
- Connection timeouts
- Authentication failures
- Rate limit exceeded
- Invalid responses
- SSL/TLS errors

## Security Considerations
- Secure credential storage
- Certificate validation
- Token management
- Audit logging
- Access control

## Testing Strategy

### Unit Tests
- REST client functionality
- Authentication methods
- Request/response handling
- Error recovery

### Integration Tests
- Vendor API compatibility
- Authentication flows
- Session management
- Rate limiting

### End-to-End Tests
- Device discovery
- Configuration management
- Capability detection
- Error scenarios

## Performance Considerations
- Connection pooling
- Request caching
- Rate limiting
- Batch operations
- Async processing

## Future Enhancements
- GraphQL support
- WebSocket connections
- Bulk operations
- Custom API extensions
- API versioning support 