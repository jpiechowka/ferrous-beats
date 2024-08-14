use axum::http::StatusCode;
use axum::response::IntoResponse;

pub struct ServerError(anyhow::Error);

impl From<anyhow::Error> for ServerError {
    fn from(value: anyhow::Error) -> Self {
        Self(value)
    }
}

impl IntoResponse for ServerError {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.0.to_string()).into_response()
    }
}
