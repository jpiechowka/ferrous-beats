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
    /// Disable DNS over HTTPS (DoH) for HTTP client. DoH provides some additional privacy compared to plain DNS.
    #[arg(long = "disable-doh")]
    pub disable_doh: bool,
    /// Your file library directory
    #[arg(short = 'l', long = "library-dir", default_value = "library")]
    pub library_dir: String,
    /// Download directory for all the used tools (yt-dlp, ffmpeg, chromparint)
    #[arg(short = 't', long = "tools-download-dir", default_value = "tools")]
    pub tools_download_dir: String,
    /// Download directory for audio files. They will be moved to library directory after successful download
    #[arg(
        short = 'a',
        long = "audio-download-dir",
        default_value = "downloads/music"
    )]
    pub audio_download_dir: String,
    /// Download directory for video files. They will be moved to library directory after successful download
    #[arg(
        short = 'v',
        long = "video-download-dir",
        default_value = "downloads/videos"
    )]
    pub video_download_dir: String,
}
