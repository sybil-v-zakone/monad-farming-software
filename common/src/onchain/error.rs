use alloy::{
    consensus::TxType,
    hex,
    network::{Ethereum, UnbuiltTransactionError},
    primitives::ruint::ParseError,
    providers::{MulticallError, PendingTransactionError},
    transports::{RpcError as RpcErr, TransportErrorKind},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ClientError {
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
    UnbuiltTx(#[from] Box<UnbuiltTransactionError<Ethereum>>),

    #[error("tx type `{0}` is not supported")]
    UnexpectedTxType(TxType),

    #[error(transparent)]
    Contract(#[from] alloy::contract::Error),

    #[error(transparent)]
    Multicall(#[from] MulticallError),
}
