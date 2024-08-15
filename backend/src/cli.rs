use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(version, about)]
pub struct Cli {
    /// Enable verbose logging
    #[arg(short, long)]
    pub verbose: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Main application command used to run the server and serve the frontend
    Run(RunCommand),
}

#[derive(Debug, Parser)]
pub struct RunCommand {
    /// Server port
    #[arg(short = 'p', long = "port", default_value_t = 13337)]
    pub port: u16,
    /// Server host
    #[arg(long = "host", default_value = "127.0.0.1")]
    pub host: String,
    /// Download directory for yt-dlp (A feature-rich command-line audio/video downloader)
    #[arg(short = 'y', long = "yt-dlp-download-dir", default_value = "yt-dlp")]
    pub dlp_download_dir: String,
    /// Download directory for audio files
    #[arg(short = 'a', long = "audio-download-dir", default_value = "music")]
    pub audio_download_dir: String,
    /// Download directory for video files
    #[arg(short = 'v', long = "video-download-dir", default_value = "videos")]
    pub video_download_dir: String,
}
