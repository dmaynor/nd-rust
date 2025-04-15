# User Management System Design Document

## Overview
The user management system provides authentication, authorization, and user management capabilities for NetDisco-rust. It implements role-based access control (RBAC) and audit logging for security and compliance.

## Components

### 1. Authentication System
- Username/password authentication with Argon2 password hashing
- JWT token-based session management
- MFA support using TOTP (Time-based One-Time Password)
- Password policy enforcement
- Account lockout protection

### 2. Authorization System
- Role-based access control (RBAC)
- Permission sets for different resource types
- Resource-level access control
- API endpoint authorization middleware
- CLI command authorization

### 3. User Management
- User CRUD operations
- Role management
- Password reset functionality
- User profile management
- Session management
- Account status management

### 4. Audit Logging
- User activity logging
- Authentication attempts
- Authorization decisions
- Administrative actions
- Data export capabilities

## Database Schema

### Users Table
```sql
CREATE TYPE user_status AS ENUM ('active', 'inactive', 'locked');

CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username VARCHAR(255) NOT NULL UNIQUE,
    email VARCHAR(255) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    full_name VARCHAR(255),
    status user_status NOT NULL DEFAULT 'active',
    mfa_enabled BOOLEAN NOT NULL DEFAULT false,
    mfa_secret VARCHAR(255),
    failed_login_attempts INTEGER NOT NULL DEFAULT 0,
    last_login TIMESTAMP WITH TIME ZONE,
    password_changed_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);
```

### Roles Table
```sql
CREATE TABLE roles (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL UNIQUE,
    description TEXT,
    is_system BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);
```

### Permissions Table
```sql
CREATE TYPE resource_type AS ENUM (
    'device', 'config', 'backup', 'report', 'user', 'role',
    'system', 'network', 'inventory'
);

CREATE TYPE permission_action AS ENUM (
    'create', 'read', 'update', 'delete', 'execute'
);

CREATE TABLE permissions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL UNIQUE,
    resource_type resource_type NOT NULL,
    action permission_action NOT NULL,
    description TEXT,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (resource_type, action)
);
```

### Role Permissions Table
```sql
CREATE TABLE role_permissions (
    role_id UUID NOT NULL REFERENCES roles(id) ON DELETE CASCADE,
    permission_id UUID NOT NULL REFERENCES permissions(id) ON DELETE CASCADE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (role_id, permission_id)
);
```

### User Roles Table
```sql
CREATE TABLE user_roles (
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role_id UUID NOT NULL REFERENCES roles(id) ON DELETE CASCADE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (user_id, role_id)
);
```

### Audit Log Table
```sql
CREATE TYPE audit_action AS ENUM (
    'login', 'logout', 'create', 'read', 'update', 'delete',
    'enable', 'disable', 'execute'
);

CREATE TABLE audit_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id) ON DELETE SET NULL,
    action audit_action NOT NULL,
    resource_type resource_type NOT NULL,
    resource_id UUID,
    details JSONB,
    ip_address INET,
    user_agent TEXT,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);
```

## API Endpoints

### Authentication
- POST /api/auth/login - Login with username/password
- POST /api/auth/logout - Logout current session
- POST /api/auth/refresh - Refresh JWT token
- POST /api/auth/mfa/enable - Enable MFA for user
- POST /api/auth/mfa/disable - Disable MFA for user
- POST /api/auth/mfa/verify - Verify MFA token

### User Management
- GET /api/users - List users
- POST /api/users - Create new user
- GET /api/users/{id} - Get user details
- PUT /api/users/{id} - Update user
- DELETE /api/users/{id} - Delete user
- POST /api/users/{id}/reset-password - Reset user password
- POST /api/users/{id}/lock - Lock user account
- POST /api/users/{id}/unlock - Unlock user account

### Role Management
- GET /api/roles - List roles
- POST /api/roles - Create new role
- GET /api/roles/{id} - Get role details
- PUT /api/roles/{id} - Update role
- DELETE /api/roles/{id} - Delete role
- GET /api/roles/{id}/permissions - List role permissions
- PUT /api/roles/{id}/permissions - Update role permissions

### Audit Logs
- GET /api/audit-logs - List audit logs
- GET /api/audit-logs/{id} - Get audit log details
- POST /api/audit-logs/export - Export audit logs

## Implementation Details

### Password Policy
- Minimum length: 12 characters
- Must contain: uppercase, lowercase, numbers, special characters
- Maximum age: 90 days
- Password history: last 12 passwords
- Account lockout after 5 failed attempts

### JWT Configuration
- Token expiration: 1 hour
- Refresh token expiration: 7 days
- Token signing using RS256 algorithm
- Token includes: user ID, roles, permissions

### Default Roles
1. Administrator
   - Full system access
   - User and role management
   - System configuration
   
2. Operator
   - Device management
   - Configuration management
   - Network operations
   
3. Viewer
   - Read-only access to devices
   - View configurations
   - View reports

### Security Considerations
- All passwords hashed using Argon2id
- Rate limiting on authentication endpoints
- JWT token rotation
- Secure session management
- Input validation and sanitization
- SQL injection prevention
- XSS protection

## Testing Strategy
1. Unit Tests
   - Authentication functions
   - Authorization checks
   - Password policy validation
   - Token management

2. Integration Tests
   - API endpoint testing
   - Database operations
   - Role and permission management
   - Audit logging

3. Security Tests
   - Password hashing
   - Token validation
   - Authorization bypass attempts
   - Rate limiting
   - SQL injection prevention
``` 