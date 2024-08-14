use crate::handlers::errors::ServerError;
use axum::http::StatusCode;
use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct HelloResponse {
    message: &'static str,
}

pub async fn hello_api() -> Result<(StatusCode, Json<HelloResponse>), ServerError> {
    let response = HelloResponse {
        message: "Hello from the Ferrous Beats! Welcome to the API.",
    };

    Ok((StatusCode::OK, Json(response)))
}
