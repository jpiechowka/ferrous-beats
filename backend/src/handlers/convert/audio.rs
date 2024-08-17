use crate::handlers::errors::ServerError;
use crate::handlers::shared::model::commands::CommandExecutionResults;
use crate::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde::Deserialize;
use tracing::{debug, instrument};

#[derive(Debug, Deserialize)]
pub struct ConvertAudioRequest {
    audio_file_path: String,
    output_format: String,
}

#[instrument(err, ret(level = "debug"), skip(app_state))]
pub async fn handle_audio_conversion(
    State(app_state): State<AppState>,
    Json(payload): Json<ConvertAudioRequest>,
) -> Result<(StatusCode, Json<CommandExecutionResults>), ServerError> {
    debug!("Handling audio conversion");

    todo!()
}
