# nd-rust

Rust port of the [Netdisco](https://github.com/netdisco/netdisco) network management tool.

## Development Setup

This project supports running the required PostgreSQL database via Docker Compose for a consistent development environment.

### Prerequisites

*   Rust toolchain (latest stable recommended)
*   Docker and Docker Compose
*   `sqlx-cli` (install via `cargo install sqlx-cli`)

### Configuration

1.  **Environment File:** Create a `.env` file in the project root (or copy `.env.example` if it exists). This file is used by `docker-compose` and potentially by `sqlx-cli` and the application.
2.  **Database Credentials:** Ensure the following variables are set in your `.env` file. They will be used to configure the PostgreSQL container and for the application to connect:
    ```dotenv
    # Used by docker-compose to setup the DB container
    POSTGRES_USER=postgres
    POSTGRES_PASSWORD=changeme
    POSTGRES_DB=nd_rust_db
    
    # Used by sqlx-cli and the Rust application (matches container setup above)
    DATABASE_URL="postgres://postgres:changeme@localhost:5433/nd_rust_db"
    ```
    *Note: If you change the `POSTGRES_*` variables, ensure `DATABASE_URL` is updated accordingly. If you change the host port mapping in `docker-compose.yml` (e.g., to something other than `5433:5432`), update the port in `DATABASE_URL`.* 

### Running the Database (Docker Compose)

1.  **Start the database container:**
    ```bash
    docker-compose up -d db 
    ```
    The `-d` runs it in the background. The first time you run this, it will download the PostgreSQL image.

2.  **Check container status:**
    ```bash
    docker-compose ps
    ```
    Wait until the `db` service state is `healthy`.

3.  **(Troubleshooting) Manually Create User if Needed:**
    The default `postgres` user should be created automatically. If you change `POSTGRES_USER` in `.env` and encounter connection issues, you might need to manually create that specific user:
    ```bash
    # Replace 'new_user' and 'new_password' with the values from your .env
    docker-compose exec -u postgres db psql -d nd_rust_db -c "CREATE ROLE new_user WITH LOGIN PASSWORD 'new_password';"
    docker-compose exec -u postgres db psql -d nd_rust_db -c "GRANT ALL PRIVILEGES ON SCHEMA public TO new_user;"
    ```

4.  **(Optional) View logs:**
    ```bash
    docker-compose logs -f db
    ```

### Database Migrations

This project uses `sqlx-cli` for database schema migrations.

1.  Ensure the database container is running and healthy (`docker-compose ps`) and that the necessary user exists (see Troubleshooting step above if needed).
2.  Run the migrations against the containerized database:
    ```bash
    sqlx migrate run 
    ```
    *Note: Your local `sqlx-cli` must be able to connect using the `DATABASE_URL` specified in `.env` (pointing to `localhost:5433` by default).* 

    This command needs to be run initially and any time new migrations are added.

3.  **(Optional) Stop the database container when done:**
    ```bash
    docker-compose stop db
    ```
    Or to stop and remove the container:
    ```bash
    docker-compose down
    ```

### Running the Application

With the database running in Docker, you can run the Rust application directly:

```bash
cargo run
```

## TODO

See [docs/todo.md](docs/todo.md) for the project task list.
See GitHub Issues for tracked items: [https://github.com/dmaynor/nd-rust/issues](https://github.com/dmaynor/nd-rust/issues) 