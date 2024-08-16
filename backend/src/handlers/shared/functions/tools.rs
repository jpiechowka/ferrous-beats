use crate::AppState;
use anyhow::Context;
use std::path::{Path, PathBuf};
use tracing::{debug, instrument};

#[instrument(err, skip(app_state))]
pub async fn get_yt_dlp_executable_path(app_state: &AppState) -> Result<PathBuf, anyhow::Error> {
    debug!("Getting yt-dlp executable path");

    let os = std::env::consts::OS;
    let dlp_dir = Path::new(&app_state.config.server_settings.tools_download_dir);
    let executable_path = dlp_dir.join(if os == "windows" {
        "yt-dlp.exe"
    } else {
        "yt-dlp"
    });
    let canonical_path = executable_path
        .canonicalize()
        .context("Failed to canonicalize yt-dlp executable path. One of the reasons can be that the executable does not exist")?;

    Ok(canonical_path)
}

#[instrument(err, skip(app_state))]
pub async fn get_ffmpeg_executable_path(app_state: &AppState) -> Result<PathBuf, anyhow::Error> {
    debug!("Getting ffmpeg executable path");

    let os = std::env::consts::OS;
    let dlp_dir = Path::new(&app_state.config.server_settings.tools_download_dir);
    let executable_path = dlp_dir.join(if os == "windows" {
        "ffmpeg.exe"
    } else {
        "ffmpeg"
    });
    let canonical_path = executable_path
        .canonicalize()
        .context("Failed to canonicalize ffmpeg executable path. One of the reasons can be that the executable does not exist")?;

    Ok(canonical_path)
}

#[instrument(err, skip(app_state))]
pub async fn get_chromaprint_fpcalc_executable_path(
    app_state: &AppState,
) -> Result<PathBuf, anyhow::Error> {
    debug!("Getting chromaprint's fpcalc executable path");

    let os = std::env::consts::OS;
    let dlp_dir = Path::new(&app_state.config.server_settings.tools_download_dir);
    let executable_path = dlp_dir.join(if os == "windows" {
        "fpcalc.exe"
    } else {
        "fpcalc"
    });
    let canonical_path = executable_path
        .canonicalize()
        .context("Failed to canonicalize chromaprint's fpcalc executable path. One of the reasons can be that the executable does not exist")?;

    Ok(canonical_path)
}

#[instrument(err)]
pub async fn get_yt_dlp_download_url_and_output_file_name(
    os: &str,
) -> Result<(&str, &str), anyhow::Error> {
    // https://github.com/yt-dlp/yt-dlp/releases
    let url = match os {
        "linux" => (
            "https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp",
            "yt-dlp",
        ),
        "windows" => (
            "https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp.exe",
            "yt-dlp.exe",
        ),
        "macos" => (
            "https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp_macos",
            "yt-dlp",
        ),
        os => anyhow::bail!("Unsupported operating system: {}", os),
    };

    Ok(url)
}

#[instrument(err)]
pub async fn get_ffmpeg_download_url_and_output_file_name(
    os: &str,
) -> Result<(&str, &str), anyhow::Error> {
    // https://ffmpeg.org/download.html
    let url = match os {
        "linux" => (
            "https://johnvansickle.com/ffmpeg/builds/ffmpeg-git-amd64-static.tar.xz",
            "ffmpeg.tar.xz",
        ),
        "windows" => (
            "https://github.com/BtbN/FFmpeg-Builds/releases/download/latest/ffmpeg-master-latest-win64-lgpl.zip",
            "ffmpeg.zip",
        ),
        "macos" => ("https://evermeet.cx/ffmpeg/ffmpeg-7.0.2.7z", "ffmpeg.7z"),
        os => anyhow::bail!("Unsupported operating system: {}", os),
    };

    Ok(url)
}

#[instrument(err)]
pub async fn get_chromaprint_download_url_and_output_file_name(
    os: &str,
) -> Result<(&str, &str), anyhow::Error> {
    // https://acoustid.org/chromaprint
    let url = match os {
        "linux" => (
            "https://github.com/acoustid/chromaprint/releases/download/v1.5.1/chromaprint-fpcalc-1.5.1-linux-x86_64.tar.gz",
            "chromaprint.tar.gz"
        ),
        "windows" => (
            "https://github.com/acoustid/chromaprint/releases/download/v1.5.1/chromaprint-fpcalc-1.5.1-windows-x86_64.zip",
            "chromaprint.zip"
        ),
        "macos" => (
            "https://github.com/acoustid/chromaprint/releases/download/v1.5.1/chromaprint-fpcalc-1.5.1-macos-universal.tar.gz",
            "chromaprint.tar.gz"
        ),
        os => anyhow::bail!("Unsupported operating system: {}", os),
    };

    Ok(url)
}
