use thiserror::Error;

use crate::modules::warmup::error::WarmupError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Warmup(#[from] WarmupError),

    #[error(transparent)]
    DatabaseError(#[from] database::error::Error),

    // dep crates
    #[error(transparent)]
    Common(#[from] common::Error),

    #[error(transparent)]
    MenuError(#[from] dialoguer::Error),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    UrlParse(#[from] url::ParseError),
}
