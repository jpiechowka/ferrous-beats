use crate::handlers::errors::ServerError;
use crate::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use tracing::{debug, instrument};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum YtDlpUpdateChannels {
    Stable,
    Master,
    Nightly,
}

#[derive(Debug, Deserialize)]
pub struct YtDlpUpdateRequest {
    update_channel: YtDlpUpdateChannels,
}

#[derive(Debug, Serialize)]
pub struct YtDlpUpdateResponse {
    stdout: Option<String>,
    stderr: Option<String>,
}

#[instrument(err, skip(app_state))]
pub async fn handle_yt_dlp_update(
    State(app_state): State<AppState>,
) -> Result<(StatusCode, Json<YtDlpUpdateResponse>), ServerError> {
    debug!("Handling updating of yt-dlp");
    todo!()
}
