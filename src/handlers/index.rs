use axum::http::StatusCode;
use axum::Json;
use serde::Serialize;
use tracing::instrument;

#[derive(Debug, Serialize)]
pub struct HelloResponse {
    message: &'static str,
}

#[instrument(ret(level = "debug"))]
pub async fn handle_api_hello() -> (StatusCode, Json<HelloResponse>) {
    let response = HelloResponse {
        message: "Hello from the Ferrous Beats! Welcome to the API.",
    };

    (StatusCode::OK, Json(response))
}
