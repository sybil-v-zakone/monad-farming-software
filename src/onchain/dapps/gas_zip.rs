use crate::onchain::client::Client as EvmClient;
use crate::{Result, onchain::error::ClientError};
use alloy::primitives::Address;
use alloy::{
    hex::FromHexError,
    network::{Ethereum, TransactionBuilder},
    primitives::{U256, address},
    providers::Provider,
    rpc::types::TransactionRequest,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GaszipError {
    #[error(transparent)]
    HexDecodeError(#[from] FromHexError),
}

const GAS_ZIP_CA: Address = address!("0x391E7C679d29bD940d63be94AD22A25d25b5A604");

pub async fn bridge<P>(evm_client: &EvmClient<P>, amount_in: U256) -> Result<bool>
where
    P: Provider<Ethereum>,
{
    let call_data = alloy::hex::decode("0x0101b1").map_err(ClientError::FromHex)?;

    let tx: TransactionRequest = TransactionRequest::default()
        .with_input(call_data)
        .to(GAS_ZIP_CA)
        .with_value(amount_in);

    evm_client.send_transaction(tx, None).await
}
