use alloy::signers::local::LocalSignerError;
use sea_orm::DbErr;
use thiserror::Error;

use crate::entity::impls::account::NewActiveModelOptionsBuilderError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Account not found")]
    NotFound,

    #[error(transparent)]
    Request(#[from] reqwest::Error),

    #[error(transparent)]
    Common(#[from] common::Error),

    // external
    #[error(transparent)]
    LocalSigner(#[from] LocalSignerError),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Db(#[from] DbErr),

    #[error(transparent)]
    Toml(#[from] toml::de::Error),

    #[error(transparent)]
    NewAccountOpts(#[from] NewActiveModelOptionsBuilderError),
}
