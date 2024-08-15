mod cli;
mod config;
mod handlers {
    pub mod download {
        pub mod audio;
        pub mod video;
    }
    pub mod yt_dlp {
        pub mod download;
        pub mod status;
        pub mod update;
    }
    pub mod errors;
    pub mod index;
    pub mod shared_model;
}

use crate::cli::{Cli, Commands};
use crate::config::Config;
use crate::handlers::download::audio::handle_audio_download;
use crate::handlers::download::video::handle_video_download;
use crate::handlers::index::handle_api_hello;
use crate::handlers::yt_dlp::download::handle_yt_dlp_download;
use crate::handlers::yt_dlp::status::handle_yt_dlp_status;
use crate::handlers::yt_dlp::update::handle_yt_dlp_update;
use anyhow::Context;
use axum::http::header;
use axum::routing::{get, post};
use axum::Router;
use clap::Parser;
use tower_http::compression::CompressionLayer;
use tower_http::cors::CorsLayer;
use tower_http::decompression::DecompressionLayer;
use tower_http::set_header::SetResponseHeaderLayer;
use tower_http::trace::{
    DefaultMakeSpan, DefaultOnFailure, DefaultOnRequest, DefaultOnResponse, TraceLayer,
};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Clone)]
struct AppState {
    config: Config,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Run(run_command) => {
            let config = config::cli_to_config(&run_command, cli.verbose)?;

            let tracing_subscriber = FmtSubscriber::builder()
                .with_max_level(config.logging_settings.level)
                .with_thread_ids(true)
                .with_thread_names(true)
                .with_ansi(true)
                .with_target(true)
                .with_file(false)
                .with_line_number(false)
                .finish();

            tracing::subscriber::set_global_default(tracing_subscriber)
                .context("Failed to set global default tracing subscriber")?;

            let trace_layer = TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().include_headers(false))
                .on_request(DefaultOnRequest::new().level(Level::DEBUG))
                .on_response(DefaultOnResponse::new().level(Level::DEBUG))
                .on_failure(DefaultOnFailure::new());

            let app_state = AppState {
                config: config.clone(),
            };

            let app = Router::new()
                .route("/", get(handle_api_hello))
                .route("/download/audio", post(handle_audio_download))
                .route("/download/video", post(handle_video_download))
                .route("/yt-dlp/download", post(handle_yt_dlp_download))
                .route("/yt-dlp/status", post(handle_yt_dlp_status))
                .route("/yt-dlp/update", post(handle_yt_dlp_update))
                .layer(tower_http::catch_panic::CatchPanicLayer::new())
                .layer(trace_layer)
                .layer(CompressionLayer::new())
                .layer(DecompressionLayer::new())
                .layer(CorsLayer::permissive())
                .layer(SetResponseHeaderLayer::overriding(
                    header::SERVER,
                    header::HeaderValue::from_str(&format!(
                        "Ferrous Beats/{} ({})",
                        APP_VERSION,
                        std::env::consts::OS,
                    ))
                    .context("Failed to configure Server header layer")?,
                ))
                .layer(SetResponseHeaderLayer::overriding(
                    header::CACHE_CONTROL,
                    header::HeaderValue::from_static("no-store"),
                ))
                .layer(SetResponseHeaderLayer::overriding(
                    header::X_FRAME_OPTIONS,
                    header::HeaderValue::from_static("DENY"),
                ))
                .layer(SetResponseHeaderLayer::overriding(
                    header::X_CONTENT_TYPE_OPTIONS,
                    header::HeaderValue::from_static("nosniff"),
                ))
                .layer(SetResponseHeaderLayer::overriding(
                    header::REFERRER_POLICY,
                    header::HeaderValue::from_static("no-referrer"),
                ))
                .layer(SetResponseHeaderLayer::overriding(
                    header::CONTENT_SECURITY_POLICY,
                    header::HeaderValue::from_static("default-src 'none'"),
                ))
                .with_state(app_state);

            let bind_addr = format!(
                "{}:{}",
                config.server_settings.host, config.server_settings.port
            );

            let listener = tokio::net::TcpListener::bind(bind_addr.clone())
                .await
                .context(format!(
                    "failed to bind TCP listener to address: {}",
                    &bind_addr
                ))?;

            info!("Ferrous Beats API is starting on {}", &bind_addr);

            axum::serve(listener, app)
                .await
                .context("axum::serve failed")?;
        }
    }

    Ok(())
}
