# CLI Interface Design Document

## Overview
This document outlines the design and implementation of the command-line interface (CLI) for NetDisco-rust. The CLI provides a powerful and user-friendly way to interact with the system through terminal commands.

## Components

### 1. Command Framework
- **Command Parser**: Processes command-line arguments and options
- **Command Registry**: Manages available commands and their handlers
- **Help System**: Provides detailed usage information and examples
- **Shell Integration**: Supports interactive shell mode with command history

### 2. Core Commands
#### Device Discovery
- `discover scan [options]` - Scan network for devices
- `discover list` - List discovered devices
- `discover show <device-id>` - Show device details
- `discover rescan <device-id>` - Rescan specific device

#### Device Management
- `device list` - List all managed devices
- `device add <address> [options]` - Add new device
- `device remove <device-id>` - Remove device
- `device update <device-id> [options]` - Update device properties
- `device enable <device-id>` - Enable device management
- `device disable <device-id>` - Disable device management

#### Configuration Management
- `config list` - List configurations
- `config show <config-id>` - Show configuration details
- `config create <name> [options]` - Create new configuration
- `config apply <config-id> <device-id>` - Apply configuration to device
- `config validate <config-id>` - Validate configuration
- `config export <config-id> <file>` - Export configuration
- `config import <file>` - Import configuration

#### NETCONF Operations
- `netconf connect <device-id>` - Establish NETCONF session
- `netconf get-config <device-id> [options]` - Get device configuration
- `netconf edit-config <device-id> <file>` - Edit device configuration
- `netconf disconnect <device-id>` - Close NETCONF session
- `netconf capabilities <device-id>` - Show device capabilities

#### Backup Management
- `backup list` - List backup configurations
- `backup create <device-id>` - Create new backup
- `backup restore <backup-id> <device-id>` - Restore from backup
- `backup export <backup-id> <file>` - Export backup
- `backup schedule <device-id> [options]` - Schedule backup

#### Firmware Management
- `firmware list` - List firmware images
- `firmware upload <file> [options]` - Upload firmware image
- `firmware update <device-id> <firmware-id>` - Update device firmware
- `firmware rollback <device-id>` - Rollback firmware update

#### Reporting
- `report generate <type> [options]` - Generate report
- `report list` - List available reports
- `report export <report-id> <file>` - Export report
- `report schedule <type> [options]` - Schedule report generation

### 3. Interactive Shell
- Command history
- Tab completion
- Context-aware help
- Persistent sessions
- Scripting support

## Implementation Details

### Command Structure
```rust
pub trait Command {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn execute(&self, args: &[String]) -> Result<()>;
    fn help(&self) -> String;
}

pub struct CommandContext {
    pub config: Config,
    pub db: Pool<Postgres>,
    pub client: Client,
}

pub struct CommandRegistry {
    commands: HashMap<String, Box<dyn Command>>,
}
```

### Shell Implementation
```rust
pub struct Shell {
    context: CommandContext,
    registry: CommandRegistry,
    history: Vec<String>,
}

impl Shell {
    pub fn new(context: CommandContext) -> Self;
    pub fn run(&mut self) -> Result<()>;
    pub fn execute_command(&mut self, input: &str) -> Result<()>;
    pub fn complete_command(&self, partial: &str) -> Vec<String>;
}
```

## Database Schema

```sql
-- Command History
CREATE TABLE command_history (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    command TEXT NOT NULL,
    args JSONB,
    status TEXT NOT NULL,
    output TEXT,
    error TEXT,
    executed_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMPTZ
);

-- Scheduled Commands
CREATE TABLE scheduled_commands (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    command TEXT NOT NULL,
    args JSONB,
    schedule TEXT NOT NULL,
    last_run TIMESTAMPTZ,
    next_run TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Command Scripts
CREATE TABLE command_scripts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL UNIQUE,
    description TEXT,
    content TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

## Error Handling
- Invalid command syntax
- Missing arguments
- Invalid options
- Command execution failures
- Database errors
- Network errors
- Authentication failures

## Security Considerations
- Command authorization
- Input validation
- Secure credential handling
- Audit logging
- Session management
- Script validation

## Testing Strategy
1. Unit tests for individual commands
2. Integration tests for command sequences
3. Shell interaction tests
4. Script execution tests
5. Error handling tests
6. Performance testing

## Dependencies
- `clap` - Command-line argument parsing
- `rustyline` - Interactive shell and line editing
- `prettytable-rs` - Formatted table output
- `dialoguer` - Interactive prompts
- `indicatif` - Progress indicators
- `console` - Terminal formatting 