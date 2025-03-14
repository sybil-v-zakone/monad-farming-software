use cli::parse_cli_args;
use logger::init_logging;
use modules::menu;

pub use crate::error::{Error, Result};

mod cli;
mod config;
mod error;
mod logger;
mod modules;

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let cli = parse_cli_args();
    init_logging(&cli.log_level);

    if let Err(e) = menu::menu().await {
        tracing::error!("Stopped with error: {e}")
    }

    Ok(())
}
