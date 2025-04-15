-- init-db.sql (Simplified)
-- Tries to create the user role ONLY.

DO
$$
BEGIN
   IF NOT EXISTS (SELECT FROM pg_catalog.pg_roles WHERE rolname = 'test_user') THEN
      RAISE NOTICE 'Creating role test_user...'; -- Added notice
      CREATE ROLE "test_user" WITH LOGIN PASSWORD 'test_password';
   ELSE
      RAISE NOTICE 'Role test_user already exists.'; -- Added notice
   END IF;
END
$$;

-- Create the database if it doesn't exist
-- The docker-entrypoint script will create the DB based on POSTGRES_DB env var.
-- We just need to ensure the owner is correct if it wasn't set automatically.
ALTER DATABASE "nd_rust_db" OWNER TO "test_user";

-- Grant all privileges
GRANT ALL PRIVILEGES ON DATABASE "nd_rust_db" TO "test_user";

-- Optional: Add extensions if needed (connect to the DB first if required by extension)
-- \connect "nd_rust_db"
-- CREATE EXTENSION IF NOT EXISTS "uuid-ossp"; 