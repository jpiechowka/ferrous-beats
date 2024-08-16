use crate::handlers::errors::ServerError;
use crate::handlers::shared_funcs::{get_yt_dlp_executable_path, run_command};
use crate::handlers::shared_model::CommandExecutionResults;
use crate::AppState;
use anyhow::Context;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use strum_macros::Display;
use tracing::{debug, info, instrument};

#[derive(Debug, Deserialize)]
pub struct YtDlpUpdateRequest {
    update_channel: YtDlpUpdateChannels,
}

#[derive(Debug, Display, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum YtDlpUpdateChannels {
    Stable,
    Master,
    Nightly,
}

#[derive(Debug, Serialize)]
pub struct YtDlpUpdateResponse {
    path: String,
    update_channel_and_tag: String,
    update_execution_results: CommandExecutionResults,
}

#[instrument(err, skip(app_state))]
pub async fn handle_yt_dlp_update(
    State(app_state): State<AppState>,
    Json(payload): Json<YtDlpUpdateRequest>,
) -> Result<(StatusCode, Json<YtDlpUpdateResponse>), ServerError> {
    debug!("Handling updating of yt-dlp");

    let yt_dlp_executable_path = get_yt_dlp_executable_path(&app_state)
        .await
        .context("Failed to get yt-dlp executable path")?;

    info!("Running yt-dlp update command");

    let update_channel_and_tag =
        prepare_update_channel_and_tag(&payload.update_channel.to_string()).await;
    let command_execution_result = run_command(
        &yt_dlp_executable_path,
        &["--update-to", &update_channel_and_tag],
    )
    .await
    .context("Failed to run yt-dlp update command")?;

    info!("yt-dlp update command was executed");

    Ok((
        if command_execution_result.command_completed_successfully {
            StatusCode::OK
        } else {
            StatusCode::BAD_REQUEST
        },
        Json(YtDlpUpdateResponse {
            path: yt_dlp_executable_path.to_string_lossy().to_string(),
            update_channel_and_tag,
            update_execution_results: command_execution_result,
        }),
    ))
}

#[instrument]
async fn prepare_update_channel_and_tag(update_channel: &str) -> String {
    let update_channel_and_tag = format!("{}@latest", update_channel.to_lowercase());
    info!(
        "Update channel and tag for yt-dlp: {}",
        &update_channel_and_tag
    );
    update_channel_and_tag
}
