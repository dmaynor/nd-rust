# NetDisco-Rust TODO List

## Phase 1: Core Infrastructure & Setup
*   [x] Initialize Rust project (`cargo new nd-rust --bin`)
*   [x] Set up workspace structure (`nd_core`, `db`, `web`, `cli` crates)
*   [ ] **Build/Runtime Issue Resolution [BLOCKER]**
    *   [ ] Investigate and fix the issue preventing new code from being executed
*   [x] Logging Framework Integration **[Blocked - Build/Run Issue]**
    *   [x] Choose and integrate a logging framework (e.g., `tracing`, `log`)
    *   [x] Configure basic logging output
    *   [x] Add initial log statements
    *   [x] Verify logging works correctly
*   [x] Configuration Management Integration **[Blocked - Build/Run Issue]**
    *   [x] Choose and integrate a configuration management approach (e.g., `config-rs`, environment variables)
    *   [x] Define basic `Settings` struct
    *   [x] Implement loading from file/env
    *   [x] Create sample configuration file (`config.yaml`)
    *   [x] Verify configuration loading works correctly
*   [x] Database Setup
    *   [x] Choose and integrate a PostgreSQL client library (e.g., `sqlx`, `diesel`)
    *   [x] Implement basic database connection pool
    *   [x] Set up database migrations (e.g., using `sqlx-cli` or `diesel_cli`)
        *   [x] Create initial migration file
    *   [x] Define initial base schema (Devices, Interfaces, basic relationships)
    *   [ ] *Post-Blocker Task:* Fix trait bounds and method calls for database queries (e.g., FromRow, bind_all)
    *   [ ] *Post-Blocker Task:* Ensure all required DB trait imports are present (e.g., sqlx::Row)

## Feature: Network Discovery & Data Models
*   [x] **Core Data Models**
    *   [x] Implement data models (Rust structs for Device, Interface, etc.)
    *   [ ] *Post-Blocker Task:* Fix struct field mismatches (e.g., hostname, sysname, vendor)
    *   [ ] *Post-Blocker Task:* Resolve type mismatches (e.g., Uuid vs Option<i64>, String vs IpAddr)
    *   [ ] *Post-Blocker Task:* Update all usages of Device/related types in dependent modules (db, api, etc.)
*   [ ] **SNMP Integration** **[PAUSED - Library Issues]**
    *   [ ] Choose and integrate an SNMP library
    *   [ ] Implement basic SNMP v2c Get functionality
*   [x] **Device Discovery** **[Blocked - SNMP]**
    *   [x] Design discovery protocols/strategy
    *   [ ] Implement basic SNMP discovery logic (fetch sysDescr, sysName, etc.) **[Blocked - SNMP]**
    *   [x] Implement storage of discovered devices into the database
    *   [ ] Implement network scanning (ping sweep / SNMP walk) **[Blocked - SNMP]**
    *   [ ] Implement device classification (sysObjectID)
*   [ ] **Interface Data Collection** **[Blocked - SNMP]**
    *   [ ] Collect interface details via SNMP (IF-MIB)
    *   [ ] Store interface data in the database, linked to devices
*   [ ] **MAC Address Tracking and Port Mapping** **[Blocked - SNMP]**
    *   [ ] Extend DB schema
    *   [ ] Implement MAC address table collection via SNMP (BRIDGE-MIB)
    *   [ ] Implement logic to map MACs to Interfaces/Ports
    *   [ ] Store MAC address history
*   [ ] **ARP/Neighbor Data Collection** **[Blocked - SNMP]**
    *   [ ] Extend DB schema
    *   [ ] Implement ARP table collection via SNMP (IP-MIB)
    *   [ ] Implement LLDP/CDP neighbor collection via SNMP
    *   [ ] Store and link neighbor data
*   [ ] **Topology Discovery**
    *   [ ] Implement Network topology discovery (using neighbor data)
*   [ ] **Discovery Manager/Worker**
    *   [ ] Design worker/job system (e.g., using `tokio::spawn` initially)
    *   [ ] Create discovery manager
    *   [ ] Add auto-discovery (based on config)
    *   [ ] *Post-Blocker Task:* Correct async/await usage and method signatures if issues arise during implementation.

## Feature: Device Interaction & Management
*   [ ] **Switch Port Control** **[Blocked - SNMP]**
    *   [ ] Research SNMP SET capabilities/risks
    *   [ ] Implement Port enable/disable (SNMP SET)
    *   [ ] Implement Port speed/duplex config (SNMP SET)
    *   [ ] Implement VLAN assignment (SNMP SET)
    *   [ ] Implement Port description management (SNMP SET)
    *   [ ] Implement PoE control (SNMP SET)
    *   [ ] Implement Port security features
    *   [ ] Implement Port mirroring capabilities
*   [ ] **Advanced Device Interfaces**
    *   [ ] SSH/CLI interface support
        *   [ ] Choose and integrate SSH library (e.g., `ssh2`)
        *   [ ] Implement command execution logic
        *   [ ] Implement screen scraping/parsing
    *   [ ] REST API support (see `design_doc_rest_api.md`)
        *   [ ] Implement REST client manager
        *   [ ] Implement API Adapters for vendors
        *   [ ] Integrate with discovery and config/backup systems
    *   [ ] NETCONF support (see `design_doc_netconf.md`)
        *   [ ] Choose and integrate NETCONF library
        *   [ ] Implement NETCONF client logic
        *   [ ] Integrate with discovery and config/backup systems
    *   [ ] Vendor-specific API integration (as needed)

## Feature: Configuration & Backup
*   [ ] **Configuration Management System** (see `design_doc_config_mgmt.md`)
    *   [ ] System architecture design
    *   [ ] Database schema creation
    *   [ ] Data models implementation
    *   [ ] Implement Config manager
    *   [ ] Implement Template system
    *   [ ] Implement Version control integration
    *   [ ] Implement Compliance checking
*   [ ] **Backup Management System** (see `design_doc_backup.md`)
    *   [ ] Design backup system (Done in doc)
    *   [ ] Create database schema
    *   [ ] Implement data models
    *   [ ] Implement backup manager
    *   [ ] Implement scheduling
    *   [ ] Implement verification
    *   [ ] Implement restore capabilities
    *   [ ] Implement storage system
    *   [ ] Implement protocol handlers (SSH, REST, SNMP)
*   [ ] **Firmware Update Capabilities** (see `design_doc_firmware.md`)
    *   [ ] System architecture design
    *   [ ] Database schema creation
    *   [ ] Data models implementation
    *   [ ] Implement Firmware manager
    *   [ ] Implement Image management/repository
    *   [ ] Implement Update scheduling
    *   [ ] Implement Rollback support
    *   [ ] Implement protocol handlers (TFTP, SCP, vendor-specific)

## Feature: Monitoring & Reporting
*   [ ] **Monitoring System** (see `design_doc_monitoring.md`)
    *   [ ] Design monitoring architecture (Done in doc)
    *   [ ] Create database schema (Postgres/TSDB)
    *   [ ] Implement data models
    *   [ ] Implement Metric collection system
    *   [ ] Implement Alert rules engine
    *   [ ] Implement Alerting (notification channels)
    *   [ ] Implement Dashboards (UI component)
    *   [ ] Implement Dashboard management
    *   [ ] Implement Threshold configuration
*   [ ] **Reporting System** (see `design_doc_reporting.md`)
    *   [ ] Design Reporting System (Done in doc)
    *   [ ] Implement Report Templates System
    *   [ ] Implement Multiple Output Formats
    *   [ ] Implement Scheduled Reports
    *   [ ] Implement Report History and Management
    *   [ ] Define/Implement Database Schema

## Feature: Inventory & Assets
*   [ ] **Enhanced Inventory Capabilities** (see `design_doc_inventory.md`)
    *   [ ] Implement Hardware inventory tracking
    *   [ ] Implement Software version tracking
    *   [ ] Implement Asset management features
    *   [ ] Implement Warranty tracking
    *   [ ] Implement Maintenance scheduling
    *   [ ] Implement Inventory reporting
    *   [ ] Implement Export functionality

## Feature: User Management & Security
*   [ ] **User Management System** (see `design_doc_user_management.md`)
    *   [ ] Design User Management System (Done in doc)
    *   [ ] Database Schema
    *   [ ] Implement Role-based Access Control (RBAC)
        *   [ ] User Authentication (Hashing, JWT)
        *   [ ] Authorization System (Middleware, Permissions)
        *   [ ] Role Management
        *   [ ] Permission System
    *   [ ] Implement Multi-factor Authentication (MFA - TOTP)
    *   [ ] Implement Password Policy Enforcement
    *   [ ] Implement Audit Logging
    *   [ ] Implement Session Management

## Feature: User Interfaces (API, Web, CLI, Viz)
*   [ ] **Backend API**
    *   [ ] Choose and integrate a web framework (e.g., `Axum`, `Actix Web`)
    *   [ ] Set up basic web server structure (`web` crate)
    *   [ ] *Post-Blocker Task:* Refactor API server setup if needed (e.g., consistency between Axum/Actix)
    *   [ ] Implement Read-Only API Endpoints
        *   [ ] `/api/devices`, `/api/devices/{id}`
        *   [ ] `/api/search/mac/{mac_address}`, `/api/search/ip/{ip_address}`
    *   [ ] Implement API Endpoints for User Management
    *   [ ] Implement API Endpoints for Port Control
    *   [ ] Implement API Endpoints for Config Management
    *   [ ] Implement API Endpoints for Backup Management
    *   [ ] Implement API Endpoints for Firmware Updates
    *   [ ] Implement API Endpoints for Monitoring
    *   [ ] Implement API Endpoints for Reporting
    *   [ ] Implement API Endpoints for Inventory
    *   [ ] *Post-Blocker Task:* Add missing API modules/functions as needed (e.g., `api::device`, `api::discovery`)
*   [ ] **Web Frontend (Basic)**
    *   [ ] Set up a minimal frontend framework
    *   [ ] Create page to list devices
    *   [ ] Create page for device details
    *   [ ] Implement basic search UI
*   [ ] **Network Visualization UI** (see `design_doc_network_viz.md`)
    *   [ ] Choose and integrate frontend graphing library
    *   [ ] Implement backend API for topology data
    *   [ ] Implement Interactive network map UI
    *   [ ] Implement Device grouping UI
    *   [ ] Implement Connection visualization
    *   [ ] Implement Real-time status updates (WebSocket?)
    *   [ ] Implement Custom layout options
    *   [ ] Implement Export functionality for diagrams
*   [ ] **Management UIs**
    *   [ ] UI for User Management
    *   [ ] UI for Port Control
    *   [ ] UI for Config Management
    *   [ ] UI for Backup Management
    *   [ ] UI for Firmware Updates
    *   [ ] UI for Monitoring (Dashboards)
    *   [ ] UI for Reporting
    *   [ ] UI for Inventory Management
*   [ ] **CLI Interface** (see `design_doc_cli.md`)
    *   [ ] System architecture design (using `clap`?)
    *   [ ] Implement CLI manager/command structure
    *   [ ] Implement core commands (discovery, device mgmt)
    *   [ ] Implement command history (interactive shell)
    *   [ ] Implement scripting support
    *   [ ] Implement scheduling capabilities
    *   [ ] Implement alias and group support

## Phase: Polish & Production Readiness
*   [ ] **Infrastructure & Packaging**
    *   [ ] Finalize Binary executables (`daemon`, `cli`, `web`, `db` tool?)
    *   [ ] Create Shell wrappers
    *   [ ] Packaging (Docker image, system packages?)
    *   [ ] Setup Docker Compose for Development Database (Issue #44)
    *   [ ] Build and execution support **[Unblocked once main build issue fixed]**
*   [ ] **Testing**
    *   [ ] Add comprehensive unit tests for all modules
    *   [ ] Add integration tests for feature interactions
    *   [ ] Add end-to-end tests simulating user workflows
    *   [ ] Set up and run `cargo test` regularly
    *   [ ] Fix any test failures
    *   [ ] Set up code coverage analysis
*   [ ] **Documentation**
    *   [ ] Review and update all design documents
    *   [ ] Write User Guide
    *   [ ] Write Developer/Contributor Guide
    *   [ ] Improve code comments and Rustdoc documentation
*   [ ] **Performance Optimization**
    *   [ ] Identify bottlenecks
    *   [ ] Optimize critical code paths
    *   [ ] Implement caching where appropriate
*   [ ] **Security Hardening**
    *   [ ] Conduct security review
    *   [ ] Address potential vulnerabilities
    *   [ ] Update dependencies

## Future Considerations / Backlog
*   [ ] **Plugin System**
    *   [ ] Design plugin architecture
    *   [ ] Create plugin manager
    *   [ ] Implement plugin API
    *   [ ] Add plugin discovery mechanism
    *   [ ] Create documentation
*   [ ] IPv6 Support
*   [ ] Advanced Alerting/Correlation Engine
*   [ ] Deeper Vendor-Specific Integrations

## Development Guidelines (Reference Only)
*   Each task should be documented in design_doc.md before implementation
*   Follow the change management process for all new features
*   Consider backward compatibility when implementing new features
*   Include appropriate testing for all new functionality 