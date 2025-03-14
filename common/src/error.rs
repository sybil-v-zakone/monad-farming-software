use alloy::primitives::ruint::FromUintError;
use thiserror::Error;

use crate::onchain::dapps::{ambient::AmbientError, bean::BeanError};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    EvmClient(#[from] crate::onchain::error::ClientError),

    #[error(transparent)]
    Bean(#[from] BeanError),

    #[error(transparent)]
    Ambient(#[from] AmbientError),

    // externals
    #[error(transparent)]
    FromUintToU128(#[from] FromUintError<u128>),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Request(#[from] rquest::Error),

    #[error(transparent)]
    Toml(#[from] toml::de::Error),
}
