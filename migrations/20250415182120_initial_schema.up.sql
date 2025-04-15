-- Add up migration script here

-- Basic enum types (if needed, adjust as necessary)
-- Example: Device Status
CREATE TYPE device_status AS ENUM (
    'up',
    'down',
    'unknown'
);

-- Devices Table
CREATE TABLE devices (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    hostname VARCHAR(255) UNIQUE, -- Or make nullable depending on requirements
    ip_address INET NOT NULL UNIQUE,
    sys_name VARCHAR(255),
    sys_descr TEXT,
    vendor VARCHAR(255),
    model VARCHAR(255),
    os_version VARCHAR(255),
    serial_number VARCHAR(255),
    status device_status DEFAULT 'unknown',
    last_seen TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Index commonly queried fields
CREATE INDEX idx_devices_hostname ON devices (hostname);
CREATE INDEX idx_devices_vendor_model ON devices (vendor, model);

-- Interfaces Table
CREATE TABLE interfaces (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    device_id UUID NOT NULL REFERENCES devices(id) ON DELETE CASCADE,
    if_index INTEGER NOT NULL,
    if_name VARCHAR(255),
    if_alias VARCHAR(255),
    if_descr VARCHAR(255),
    if_type VARCHAR(100), -- e.g., ethernetCsmacd, propVirtual
    mac_address MACADDR,
    ip_address INET, -- Primary IP if known, could be nullable or moved to separate ip_addresses table
    admin_status VARCHAR(50), -- e.g., up, down, testing
    oper_status VARCHAR(50), -- e.g., up, down, testing, unknown, dormant
    speed BIGINT, -- Speed in bits per second
    mtu INTEGER,
    last_changed TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (device_id, if_index) -- Ensure unique interface per device
);

-- Index commonly queried fields
CREATE INDEX idx_interfaces_device_id ON interfaces (device_id);
CREATE INDEX idx_interfaces_mac_address ON interfaces (mac_address);

-- Trigger to automatically update updated_at timestamp (Optional but recommended)
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
   NEW.updated_at = NOW();
   RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_devices_updated_at
BEFORE UPDATE ON devices
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_interfaces_updated_at
BEFORE UPDATE ON interfaces
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();
