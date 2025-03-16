use alloy::primitives::{Address, utils::UnitsError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WarmupError {
    #[error("no more actions left for `{0}`")]
    NoActionsLeft(Address),

    #[error("no non-zero tokens at `{0}`")]
    EmptyWallet(Address),

    #[error(transparent)]
    FormatUnits(#[from] UnitsError),
}
