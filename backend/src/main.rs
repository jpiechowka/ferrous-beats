mod cli;
mod config;
mod errors;

use crate::cli::{Cli, Commands};
use crate::errors::ServerError;
use anyhow::Context;
use axum::http::StatusCode;
use axum::routing::get;
use axum::{Json, Router};
use clap::Parser;
use serde::Serialize;
use tower_http::compression::CompressionLayer;
use tower_http::trace::{
    DefaultMakeSpan, DefaultOnFailure, DefaultOnRequest, DefaultOnResponse, TraceLayer,
};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

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
                .make_span_with(DefaultMakeSpan::new().include_headers(true))
                .on_request(DefaultOnRequest::new().level(Level::DEBUG))
                .on_response(DefaultOnResponse::new().level(Level::DEBUG))
                .on_failure(DefaultOnFailure::new());

            let app = Router::new()
                .route("/", get(hello_json))
                .layer(tower_http::catch_panic::CatchPanicLayer::new())
                .layer(trace_layer)
                .layer(CompressionLayer::new());

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

#[derive(Serialize)]
struct Response {
    message: &'static str,
}

async fn hello_json() -> Result<(StatusCode, Json<Response>), ServerError> {
    let response = Response {
        message: "Hello from Ferrous Beats! Welcome to the API.",
    };

    Ok((StatusCode::OK, Json(response)))
}
