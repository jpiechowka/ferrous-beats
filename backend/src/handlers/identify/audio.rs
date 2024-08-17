use crate::handlers::errors::ServerError;
use crate::handlers::shared::functions::commands::run_command;
use crate::handlers::shared::functions::tools::get_chromaprint_fpcalc_executable_path;
use crate::handlers::shared::model::acoustid::AcoustIDApiLookupResponse;
use crate::handlers::shared::model::commands::CommandExecutionResults;
use crate::handlers::shared::model::musicbrainz::MusicbrainzAPIRecordingResponse;
use crate::AppState;
use anyhow::Context;
use axum::extract::State;
use axum::http::header::USER_AGENT;
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use tokio::fs::metadata;
use tracing::{debug, error, info, instrument};

const ACOUSTID_API_URL: &str = "https://api.acoustid.org/v2/lookup";
const MUSICBRAINZ_API_URL: &str = "https://musicbrainz.org/ws/2";
const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
const APP_REPOSITORY_URL: &str = env!("CARGO_PKG_REPOSITORY");

#[derive(Debug, Deserialize)]
pub struct IdentifyAudioRequest {
    audio_file_path: String,
}

#[derive(Debug, Serialize)]
pub struct IdentifyAudioResponse {
    musicbrainz_response: Option<MusicbrainzAPIRecordingResponse>,
    acoustid_response: Option<AcoustIDApiLookupResponse>,
    fpcalc_fingerprint: Option<FpcalcFingerprintingResult>,
    fingerprinting_command_result: CommandExecutionResults,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FpcalcFingerprintingResult {
    duration: f64,
    fingerprint: String,
}

#[instrument(err, ret(level = "debug"), skip(app_state))]
pub async fn handle_identify_audio(
    State(app_state): State<AppState>,
    Json(payload): Json<IdentifyAudioRequest>,
) -> Result<(StatusCode, Json<IdentifyAudioResponse>), ServerError> {
    debug!("Handling identification of music track");

    metadata(&payload.audio_file_path).await.context("Failed to get metadata of the audio file. Most likely the path is wrong or the file does not exist")?;

    let fpcalc_executable_path = get_chromaprint_fpcalc_executable_path(&app_state)
        .await
        .context("Failed to get chromaprint's fpcalc executable path")?;

    info!(
        "Fingerprinting audio file with chromaprint's fpcalc: {}",
        &payload.audio_file_path
    );

    let command_execution_results = run_command(
        &fpcalc_executable_path,
        &["-json", &payload.audio_file_path],
    )
    .await
    .context("Failed to fingerprint audio with chromaprint's fpcalc")?;

    if !command_execution_results.command_completed_successfully {
        error!("Failed to fingerprint audio with chromaprint's fpcalc");
        return Ok((
            StatusCode::BAD_REQUEST,
            Json(IdentifyAudioResponse {
                musicbrainz_response: None,
                acoustid_response: None,
                fpcalc_fingerprint: None,
                fingerprinting_command_result: command_execution_results,
            }),
        ));
    }

    info!("Parsing fingerprinting results as JSON");
    let fingerprinting_result: FpcalcFingerprintingResult = serde_json::from_str(
        &command_execution_results
            .stdout
            .clone()
            .context("Failed to use stdout from fpcalc as serde_json input")?,
    )
    .context("JSON parsing failed")?;

    info!("Querying AcoustID API for track identification information");
    let resp = app_state
        .http_client
        .get(ACOUSTID_API_URL)
        .query(&[
            ("client", "IVmzA2lk9AQ"), // TODO: Proper API key goes here
            ("meta", "recordingids"),
            (
                "duration",
                // TODO: Is trunc here correct?
                &fingerprinting_result.duration.trunc().to_string(),
            ),
            ("fingerprint", &fingerprinting_result.fingerprint),
        ])
        .send()
        .await
        .context("Error sending request to AcoustID API")?;

    // TODO: check returned score and handle matches with very low scores?
    let acoustid_response: AcoustIDApiLookupResponse = resp
        .json()
        .await
        .context("Failed to parse AcoustID API response as JSON")?;

    // TODO: Handle multiple IDs better?
    let best_result = acoustid_response
        .results
        .iter()
        .max_by(|a, b| a.score.partial_cmp(&b.score).unwrap_or(Ordering::Equal))
        .context("No results found in AcoustID API response")?;

    info!(
        "Best AcoustID match: {} with score {}",
        best_result.id, best_result.score
    );

    let recording_id = best_result
        .recordings
        .first()
        .map(|rec| &rec.id)
        .context("No recording ID found in the best AcoustID API result")?;

    info!("Querying MusicBrainz API for track identification information");

    let musicbrainz_user_agent =
        format!("Ferrous Beats/{} ( {} )", APP_VERSION, APP_REPOSITORY_URL);
    info!("Using MusicBrainz User-Agent: {}", musicbrainz_user_agent);

    let resp = app_state
        .http_client
        .get(format!(
            "{}/recording/{}",
            MUSICBRAINZ_API_URL, recording_id
        ))
        .header(USER_AGENT, musicbrainz_user_agent)
        .query(&[("fmt", "json"), ("inc", "artists")])
        .send()
        .await
        .context("Error sending request to MusicBrainz API")?;

    let musicbrainz_response: MusicbrainzAPIRecordingResponse = resp
        .json()
        .await
        .context("Failed to parse Musicbrainz API response as JSON")?;

    Ok((
        StatusCode::OK,
        Json(IdentifyAudioResponse {
            musicbrainz_response: Some(musicbrainz_response),
            acoustid_response: Some(acoustid_response),
            fpcalc_fingerprint: Some(fingerprinting_result),
            fingerprinting_command_result: command_execution_results,
        }),
    ))
}
