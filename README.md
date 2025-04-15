# nd-rust

Rust port of the [Netdisco](https://github.com/netdisco/netdisco) network management tool.

## Development Setup

### Prerequisites

*   Rust toolchain (latest stable recommended)
*   PostgreSQL database server
*   `sqlx-cli` (install via `cargo install sqlx-cli`)

### Configuration

1.  Copy `config.yaml.example` to `config.yaml` (if example exists) or review the existing `config.yaml`.
2.  Create a `.env` file in the project root.
3.  Add your database connection URL to the `.env` file:
    ```dotenv
    DATABASE_URL="postgres://YOUR_USER:YOUR_PASSWORD@YOUR_HOST/YOUR_DB_NAME"
    ```
    Replace `YOUR_USER`, `YOUR_PASSWORD`, `YOUR_HOST`, and `YOUR_DB_NAME` with your actual database credentials and name (e.g., `nd_rust_db`). The user needs privileges to create tables and types.

### Database Migrations

This project uses `sqlx-cli` for database schema migrations.

1.  Ensure your PostgreSQL server is running and the database specified in `DATABASE_URL` exists.
2.  Run the migrations:
    ```bash
    sqlx migrate run
    ```

    This command needs to be run initially and any time new migrations are added.

### Running the Application

```bash
cargo run
```

## TODO

See [docs/todo.md](docs/todo.md) for the project task list.
See GitHub Issues for tracked items: [https://github.com/dmaynor/nd-rust/issues](https://github.com/dmaynor/nd-rust/issues) 