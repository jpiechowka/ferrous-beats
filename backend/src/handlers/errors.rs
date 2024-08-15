use axum::http::StatusCode;
use axum::response::IntoResponse;

pub struct ServerError(anyhow::Error);

impl<E> From<E> for ServerError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
impl IntoResponse for ServerError {
    fn into_response(self) -> axum::response::Response {
        let error_chain = self
            .0
            .chain()
            .map(|e| format!("    {}", e))
            .collect::<Vec<_>>()
            .join("\n");

        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!(
                "Something went wrong: {}\n\nError chain:\n{}\n",
                self.0, error_chain
            ),
        )
            .into_response()
    }
}
