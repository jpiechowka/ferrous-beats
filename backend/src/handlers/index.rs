use axum::http::StatusCode;
use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct HelloResponse {
    message: &'static str,
}

pub async fn hello_api() -> (StatusCode, Json<HelloResponse>) {
    let response = HelloResponse {
        message: "Hello from the Ferrous Beats! Welcome to the API.",
    };

    (StatusCode::OK, Json(response))
}
