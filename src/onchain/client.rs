use alloy::{
    hex::encode_prefixed,
    network::{Network, ReceiptResponse, TransactionBuilder},
    providers::{
        Provider,
        fillers::{FillProvider, TxFiller},
    },
    signers::{Signer, local::PrivateKeySigner},
};
use std::sync::Arc;
use tracing::instrument;

use super::error::{Error, Result};

pub struct EvmClient<F, P, N>
where
    F: TxFiller<N>,
    P: Provider<N>,
    N: Network,
{
    pub signer: PrivateKeySigner,
    pub provider: Arc<FillProvider<F, P, N>>,
}

impl<F, P, N> EvmClient<F, P, N>
where
    F: TxFiller<N>,
    P: Provider<N>,
    P: Provider<N>,
    N: Network,
{
    pub fn new(signer: PrivateKeySigner, provider: Arc<FillProvider<F, P, N>>) -> Self {
        Self { signer, provider }
    }

    #[instrument(skip_all)]
    pub async fn send_transaction(&self, mut tx: N::TransactionRequest) -> Result<bool> {
        tx.set_from(self.signer.address());

        let receipt = self
            .provider
            .send_transaction(tx)
            .await
            .map_err(Error::Rpc)?
            .get_receipt()
            .await
            .map_err(Error::PendingTx)?;

        let status = receipt.status();
        let tx_hash = format!("https://polygonscan.com/tx/{}", receipt.transaction_hash());

        match status {
            true => {
                tracing::info!("Transaction successful: {tx_hash}")
            }
            false => {
                tracing::error!("Transaction failed: {tx_hash}")
            }
        }

        Ok(status)
    }

    pub async fn get_signature(&self, message: &String) -> Result<String> {
        let signature = self
            .signer
            .sign_message(message.as_bytes())
            .await
            .map_err(Error::Signer)?;
        let signature = encode_prefixed(signature.as_bytes());
        Ok(signature)
    }
}
