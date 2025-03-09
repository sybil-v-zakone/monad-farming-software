use alloy::{
    hex,
    primitives::ruint::ParseError,
    providers::PendingTransactionError,
    transports::{RpcError as RpcErr, TransportErrorKind},
};
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Rpc(#[from] RpcErr<TransportErrorKind>),

    #[error(transparent)]
    PendingTx(#[from] PendingTransactionError),

    #[error(transparent)]
    Signer(#[from] alloy::signers::Error),

    #[error(transparent)]
    Parse(#[from] ParseError),

    #[error(transparent)]
    FromHex(#[from] hex::FromHexError),

    #[error(transparent)]
    Request(#[from] rquest::Error),
}
