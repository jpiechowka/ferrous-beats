mod errors;

use anyhow::Context;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{Json, Router};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = Router::new()
        .route("/", get(hello_json))
        .layer(tower_http::catch_panic::CatchPanicLayer::new());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .context("failed to bind TCP listener")?;
    axum::serve(listener, app)
        .await
        .context("axum::serve failed")?;

    Ok(())
}

async fn hello_json() -> Result<(StatusCode, Json<Response>), ServerError> {
    let response = Response {
        message: generate_message().context("failed to generate message")?,
    };

    Ok((StatusCode::OK, Json(response)))
}

struct ServerError(anyhow::Error);

// This allows ? to automatically convert anyhow::Error to AppError
impl From<anyhow::Error> for ServerError {
    fn from(value: anyhow::Error) -> Self {
        Self(value)
    }
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.0.to_string()).into_response()
    }
}
