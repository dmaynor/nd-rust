-- Add down migration script here

-- Drop triggers first if they exist
DROP TRIGGER IF EXISTS update_interfaces_updated_at ON interfaces;
DROP TRIGGER IF EXISTS update_devices_updated_at ON devices;
DROP FUNCTION IF EXISTS update_updated_at_column();

-- Drop tables (order matters due to foreign keys)
DROP TABLE IF EXISTS interfaces;
DROP TABLE IF EXISTS devices;

-- Drop types (if created)
DROP TYPE IF EXISTS device_status;
