use crate::handlers::errors::ServerError;
use crate::AppState;
use anyhow::Context;
use axum::extract::State;
use axum::http::header::CONTENT_TYPE;
use axum::http::StatusCode;
use axum::response::Response;
use axum::{body::Body, extract::Path};
use tokio::fs::{metadata, File};
use tokio_util::io::ReaderStream;
use tracing::{debug, instrument};

#[instrument(err, ret(level = "debug"), skip(app_state))]
pub async fn handle_play_audio(
    Path(library_file_name): Path<String>,
    State(app_state): State<AppState>,
) -> Result<(StatusCode, Response<Body>), ServerError> {
    debug!("Handling playing of music track");

    let library_dir = std::path::Path::new(&app_state.config.library_settings.dir);
    let audio_file_path = library_dir.join(&library_file_name);

    metadata(&audio_file_path)
        .await
        .context("Failed to get metadata of the audio file. Most likely the path is wrong or the file does not exist")?;

    let file = File::open(&audio_file_path).await.context(format!(
        "Failed to open the audio file: {}",
        &audio_file_path.display()
    ))?;

    let stream = ReaderStream::new(file);
    let response_body = Body::from_stream(stream);

    // TODO: auto detection / more entries?
    // Set Content-Type based on file extension
    let content_type = match audio_file_path
        .extension()
        .context("Failed to get audio file extension")?
        .to_str()
    {
        Some("mp3") => "audio/mpeg",
        Some("ogg") => "audio/ogg",
        Some("opus") => "audio/opus",
        _ => "application/octet-stream", // fallback
    };

    let response = Response::builder()
        .status(StatusCode::OK)
        .header(CONTENT_TYPE, content_type)
        .body(response_body)
        .context("Failed to build audio streaming response")?;

    Ok((StatusCode::OK, response))
}
