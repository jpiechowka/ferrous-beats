use crate::handlers::errors::ServerError;
use crate::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use tracing::{debug, instrument};

#[derive(Debug, Deserialize)]
pub struct IdentifyAudioRequest {}

#[derive(Debug, Serialize)]
pub struct IdentifyAudioResponse {}

#[instrument(err, ret(level = "debug"), skip(app_state))]
pub async fn handle_identify_audio(
    State(app_state): State<AppState>,
    Json(payload): Json<IdentifyAudioRequest>,
) -> Result<(StatusCode, Json<IdentifyAudioResponse>), ServerError> {
    debug!("Handling identification of music track");

    // TODO: Use fpcalc binary to fingerprint the audio file
    // TODO: Fire API request to get MusicBrainz track ID, then details
    // TODO: Return details to the user

    todo!()
}
