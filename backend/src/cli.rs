use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
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

#[derive(Parser, Debug)]
pub struct RunCommand {
    /// Server port
    #[arg(short = 'p', long = "port", default_value_t = 13337)]
    pub port: u16,
    /// Server host
    #[arg(short = 'a', long = "host", default_value = "127.0.0.1")]
    pub host: String,
}
