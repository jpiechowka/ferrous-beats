use crate::cli;
use tracing::Level;

#[derive(Debug, Clone)]
pub struct Config {
    pub server_settings: ServerSettings,
    pub library_settings: LibrarySettings,
    pub audio_download_settings: AudioDownloadSettings,
    pub video_download_settings: VideoDownloadSettings,
    pub logging_settings: LoggingSettings,
}

#[derive(Debug, Clone)]
pub struct ServerSettings {
    pub port: u16,
    pub host: String,
    pub tools_download_dir: String,
}

#[derive(Debug, Clone)]
pub struct LibrarySettings {
    pub dir: String,
}

#[derive(Debug, Clone)]
pub struct AudioDownloadSettings {
    pub download_dir: String,
}

#[derive(Debug, Clone)]
pub struct VideoDownloadSettings {
    pub download_dir: String,
}

#[derive(Debug, Clone)]
pub struct LoggingSettings {
    pub level: Level,
}

pub fn cli_to_config(
    run_command: &cli::RunCommand,
    is_verbose_logging_enabled: bool,
) -> anyhow::Result<Config> {
    let logging_level = if is_verbose_logging_enabled {
        Level::DEBUG
    } else {
        Level::INFO
    };
    let logging_settings = LoggingSettings {
        level: logging_level,
    };

    let server_settings = ServerSettings {
        port: run_command.port,
        host: run_command.host.clone(),
        tools_download_dir: run_command.tools_download_dir.clone(),
    };

    let library_settings = LibrarySettings {
        dir: run_command.library_dir.clone(),
    };

    let audio_download_settings = AudioDownloadSettings {
        download_dir: run_command.audio_download_dir.clone(),
    };

    let video_download_settings = VideoDownloadSettings {
        download_dir: run_command.video_download_dir.clone(),
    };

    Ok(Config {
        server_settings,
        library_settings,
        audio_download_settings,
        video_download_settings,
        logging_settings,
    })
}
