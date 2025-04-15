use axum::{
    routing::get,
    response::{IntoResponse, Response, Json},
    extract::State,
    http::StatusCode,
    Router,
};
use db::{PgPool, DbError, Device};
use nd_core::Settings;
use std::net::SocketAddr;
use tower_http::trace::{TraceLayer, DefaultMakeSpan};
use tower_http::cors::{CorsLayer, Any};

// Define an AppState that holds the database pool
#[derive(Clone)]
struct AppState {
    db_pool: PgPool,
}

// Define a custom error type for API responses
#[derive(Debug)]
enum ApiError {
    DbError(DbError),
    InternalError(String),
}

// Implement IntoResponse for ApiError to convert errors into HTTP responses
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            ApiError::DbError(db_error) => match db_error {
                DbError::NotFound => (StatusCode::NOT_FOUND, "Resource not found".to_string()),
                DbError::QueryFailed(e) => {
                    tracing::error!(error = %e, "Database query failed");
                    (StatusCode::INTERNAL_SERVER_ERROR, "Database error".to_string())
                }
                // Add other DbError variants as needed
                _ => (StatusCode::INTERNAL_SERVER_ERROR, "An unexpected database error occurred".to_string()),
            },
            ApiError::InternalError(msg) => {
                tracing::error!(error = %msg, "Internal server error");
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string())
            }
        };
        (status, Json(serde_json::json!({ "error": error_message }))).into_response()
    }
}

// Implement From<DbError> for ApiError
impl From<DbError> for ApiError {
    fn from(err: DbError) -> Self {
        ApiError::DbError(err)
    }
}

// Basic root handler for health check
async fn root_handler() -> &'static str {
    "nd-rust API Server is running"
}

// Handler to list devices
async fn list_devices_handler(
    State(state): State<AppState>
) -> Result<Json<Vec<Device>>, ApiError> {
    tracing::info!("Handling request for /api/devices");
    let devices = db::list_devices(&state.db_pool).await?;
    Ok(Json(devices))
}

// Function to create and run the Axum server
pub async fn run_server(pool: PgPool, settings: &Settings) -> Result<(), Box<dyn std::error::Error>> {
    let app_state = AppState { db_pool: pool };

    // Define API routes
    let api_routes = Router::new()
        .route("/devices", get(list_devices_handler));
        // Add more API routes here later

    // Define the main application router
    let app = Router::new()
        .route("/", get(root_handler)) // Health check
        .nest("/api", api_routes) // Mount API routes under /api
        .with_state(app_state)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(tracing::Level::INFO))
                .on_response(()), // Customize later if needed
        )
        // Add CORS layer - adjust origins as needed for production
        .layer(CorsLayer::new().allow_origin(Any).allow_methods(Any)); 

    // Define the address to bind to - consider making this configurable later
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("API server listening on {}", addr);

    // Run the server
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

// Remove default lib content if present
/*
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
*/
