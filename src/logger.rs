use tracing::Level;
use tracing_subscriber::fmt::SubscriberBuilder;

use crate::cli::LogLevel;

pub fn init_logging(log_level: &LogLevel) {
    let level = match log_level {
        LogLevel::Error => Level::ERROR,
        LogLevel::Warn => Level::WARN,
        LogLevel::Info => Level::INFO,
        LogLevel::Debug => Level::DEBUG,
        LogLevel::Trace => Level::TRACE,
    };

    SubscriberBuilder::default()
        .with_file(true)
        .with_thread_ids(true)
        .pretty()
        .with_max_level(level)
        .init();
}
