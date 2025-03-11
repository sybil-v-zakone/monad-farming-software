use alloy::primitives::ruint::FromUintError;
use thiserror::Error;

use crate::onchain::dapps::{ambient::AmbientError, bean::BeanError};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    EvmClient(#[from] crate::onchain::error::ClientError),

    #[error(transparent)]
    Bean(#[from] BeanError),

    #[error(transparent)]
    Ambient(#[from] AmbientError),

    #[error(transparent)]
    FromUintToU128(#[from] FromUintError<u128>),
}
