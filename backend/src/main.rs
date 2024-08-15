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
}

use crate::cli::{Cli, Commands};
use crate::config::Config;
use crate::handlers::index::handle_api_hello;
use crate::handlers::yt_dlp::download::handle_yt_dlp_download;
use crate::handlers::yt_dlp::status::handle_yt_dlp_status;
use anyhow::Context;
use axum::routing::get;
use axum::Router;
use clap::Parser;
use tower_http::compression::CompressionLayer;
use tower_http::trace::{
    DefaultMakeSpan, DefaultOnFailure, DefaultOnRequest, DefaultOnResponse, TraceLayer,
};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

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
                // TODO: Change to POST?
                .route("/yt-dlp/download", get(handle_yt_dlp_download))
                .route("/yt-dlp/status", get(handle_yt_dlp_status))
                .layer(tower_http::catch_panic::CatchPanicLayer::new())
                .layer(trace_layer)
                .layer(CompressionLayer::new())
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
