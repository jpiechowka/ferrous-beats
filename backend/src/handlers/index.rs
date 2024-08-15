use axum::http::StatusCode;
use axum::Json;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct HelloResponse {
    message: &'static str,
}

pub async fn handle_api_hello() -> (StatusCode, Json<HelloResponse>) {
    let response = HelloResponse {
        message: "Hello from the Ferrous Beats! Welcome to the API.",
    };

    (StatusCode::OK, Json(response))
}
