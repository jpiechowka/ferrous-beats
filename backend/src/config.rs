use crate::cli;
use tracing::Level;

#[derive(Debug)]
pub struct Config {
    pub server_settings: ServerSettings,
    pub logging_settings: LoggingSettings,
}

#[derive(Debug)]
pub struct ServerSettings {
    pub port: u16,
    pub host: String,
}

#[derive(Debug)]
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
    };

    Ok(Config {
        logging_settings,
        server_settings,
    })
}
