use clap::{Parser, ValueEnum};

#[derive(ValueEnum, Clone, Debug, Default)]
pub enum LogLevel {
    Error,
    Warn,
    #[default]
    Info,
    Debug,
    Trace,
}

/// cargo run -- -l error | warn | info | debug | trace
#[derive(Parser, Debug, Default)]
#[command(about = "Sets the level of logging (error, warn, info, debug, trace)")]
pub struct Cli {
    #[arg(short, long, value_enum, default_value_t = LogLevel::Info)]
    pub log_level: LogLevel,
}

pub fn parse_cli_args() -> Cli {
    Cli::try_parse().unwrap_or_else(|e| {
        eprintln!("Invalid CLI args provided: {e}. Using default");
        Cli::default()
    })
}
