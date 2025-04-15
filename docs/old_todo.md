# NetDisco-Rust TODO List

## Completed Features ✅

### Network Discovery & Monitoring
- ✅ MAC address tracking and port mapping
  - [x] MAC address table collection via SNMP
  - [x] Port mapping functionality
  - [x] MAC address history tracking
  - [x] MAC address search functionality
  - [x] Port status monitoring
  - [x] Port utilization tracking

- ✅ Device Discovery
  - [x] Design discovery protocols
  - [x] Create database schema
  - [x] Implement data models
  - [x] Create discovery manager
  - [x] Add auto-discovery
  - [x] Implement device profiling
  - [x] SNMP discovery support
  - [x] Network scanning
  - [x] Topology discovery
  - [x] Device classification

- ✅ Switch port control
  - [x] Port enable/disable functionality
  - [x] Port speed and duplex configuration
  - [x] VLAN assignment interface
  - [x] Port description management
  - [x] PoE control for supported devices
  - [x] Port security features
  - [x] Port mirroring capabilities

### User Interface
- ✅ Network visualization
  - [x] Network topology discovery
  - [x] Interactive network map
  - [x] Device grouping functionality
  - [x] Connection visualization
  - [x] Real-time status updates
  - [x] Custom layout options
  - [x] Export functionality for network diagrams

### Device Management
- ✅ Device interfaces
  - [x] SSH/CLI interface support
  - [x] REST API support for modern devices
  - [x] NETCONF support
  - [x] Vendor-specific API integration
  - [x] Device backup functionality

- ✅ Configuration management system
  - [x] System architecture
  - [x] Database schema
  - [x] Data models
  - [x] Config manager
  - [x] Template system
  - [x] Version control
  - [x] Compliance checking

- ✅ Firmware update capabilities
  - [x] System architecture
  - [x] Database schema
  - [x] Data models
  - [x] Firmware manager
  - [x] Image management
  - [x] Update scheduling
  - [x] Rollback support

- ✅ CLI interface
  - [x] System architecture
  - [x] Database schema
  - [x] Data models
  - [x] CLI manager
  - [x] Command history
  - [x] Scripting support
  - [x] Scheduling capabilities
  - [x] Alias and group support

### Infrastructure
- ✅ Binary executables
  - [x] netdisco-daemon for network discovery and monitoring
  - [x] netdisco-cli for command-line interface
  - [x] netdisco-web for web interface and API
  - [x] netdisco-db for database management
  - [x] Shell wrappers for all binaries
  - [x] Build and execution support

### Inventory Management
- ✅ Enhanced inventory capabilities
  - [x] Hardware inventory tracking
  - [x] Software version tracking
  - [x] Asset management features
  - [x] Warranty tracking
  - [x] Maintenance scheduling
  - [x] Inventory reporting
  - [x] Export functionality

### Configuration System
- ✅ Configuration management
  - [x] YAML configuration format
  - [x] Configuration file parsing
  - [x] Environment-specific configurations
  - [x] Configuration validation
  - [x] Secrets management
  - [x] Configuration backup/restore
  - [x] Configuration versioning

### Backup Management
- ✅ Backup system
  - [x] Design backup system
  - [x] Create database schema
  - [x] Implement data models
  - [x] Create backup manager
  - [x] Add scheduling
  - [x] Implement verification
  - [x] Add restore capabilities
  - [x] Storage system
  - [x] Protocol handlers
  - [x] API endpoints

### Reporting System
- [x] Report Templates
- [x] Multiple Output Formats (PDF, HTML, CSV, JSON, XLSX)
- [x] Scheduled Reports
- [x] Report History and Management

### User Management
- [x] Role-based Access Control
  - [x] User Authentication
  - [x] Authorization System
  - [x] Role Management
  - [x] Permission System
- [x] Multi-factor Authentication
- [x] Password Policy Enforcement
- [x] Audit Logging
- [x] Session Management
- [x] User Management UI
  - [x] User List and Management Interface
  - [x] Role Management Interface
  - [x] Audit Log Viewer
  - [x] User Profile Management
  - [x] MFA Configuration UI

- [x] Add monitoring system
  - [x] Design monitoring architecture
  - [x] Create database schema
  - [x] Implement data models
  - [x] Create monitoring manager
  - [x] Add alerting
  - [x] Implement dashboards
  - [x] Metric collection system
  - [x] Alert rules engine
  - [x] Dashboard management
  - [x] Threshold configuration

## Remaining Tasks

### Medium Priority Tasks
- [ ] Add plugin system
  - [ ] Design plugin architecture
  - [ ] Create plugin manager
  - [ ] Implement plugin API
  - [ ] Add plugin discovery
  - [ ] Create documentation

## Prioritized TODO List (2025-04-15)

- [ ] Fix struct field mismatches in Device and related models (e.g., hostname, sysname, vendor, etc.)
- [ ] Update all usages of Device and related types in db, discovery, and api modules to match the actual struct definitions
- [ ] Resolve type mismatches (Uuid vs Option<i64>, String vs IpAddr, etc.)
- [ ] Add missing modules/functions (e.g., api::device, api::discovery, api::port_mapping, configure functions)
- [ ] Correct async/await usage and method signatures
- [ ] Ensure all required trait imports are present (e.g., sqlx::Row)
- [ ] Fix trait bounds and method calls for database queries (FromRow, bind_all, etc.)
- [ ] Refactor API server setup to use consistent framework (Axum or Actix, not both)
- [ ] Add/complete tests for all modules after build errors are resolved
- [ ] Run `cargo test` and fix any test failures
- [ ] Review and update documentation as needed

## Development Guidelines
- Each task should be documented in design_doc.md before implementation
- Follow the change management process for all new features
- Consider backward compatibility when implementing new features
- Include appropriate testing for all new functionality