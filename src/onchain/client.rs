use alloy::{
    consensus::{SignableTransaction, TxEnvelope, TxType, TypedTransaction},
    hex::encode_prefixed,
    network::{Ethereum, ReceiptResponse, TransactionBuilder, TxSigner},
    primitives::{Address, U256},
    providers::Provider,
    rpc::types::TransactionRequest,
    signers::{Signer, local::PrivateKeySigner},
    sol,
    sol_types::SolCall,
};
use alloy_chains::Chain;

use super::{error::ClientError, token::Token};
use crate::Result;

sol! {
    #[sol(rpc)]
    interface IERC20 {
        function approve(address spender, uint256 amount) external returns (bool);

        function allowance(address owner, address spender) external view returns (uint256);
    }
}

pub struct Client<P, N = StrictNonceManager>
where
    P: Provider<Ethereum>,
{
    pub chain: Chain,
    pub provider: P,
    pub signer: PrivateKeySigner,
    nonce_manager: N,
}

impl<P, N> Client<P, N>
where
    P: Provider<Ethereum>,
    N: Default + ClientNonceManager<P>,
{
    pub fn new(signer: PrivateKeySigner, chain: Chain, provider: P) -> Self {
        Self {
            chain,
            provider,
            signer,
            nonce_manager: N::default(),
        }
    }

    async fn sign_tx_request(&self, tx: TransactionRequest) -> Result<TxEnvelope> {
        let unsigned_tx = tx
            .build_unsigned()
            .map_err(|e| ClientError::UnbuiltTx(Box::new(e)))?;

        match unsigned_tx {
            TypedTransaction::Legacy(mut t) => {
                let sig = self
                    .signer
                    .sign_transaction(&mut t)
                    .await
                    .map_err(ClientError::Signer)?;
                Ok(t.into_signed(sig).into())
            }
            TypedTransaction::Eip1559(mut t) => {
                let sig = self
                    .signer
                    .sign_transaction(&mut t)
                    .await
                    .map_err(ClientError::Signer)?;
                Ok(t.into_signed(sig).into())
            }
            _ => Err(crate::Error::EvmClient(ClientError::UnexpectedTxType(
                unsigned_tx.tx_type(),
            ))),
        }
    }

    pub async fn send_transaction(
        &self,
        tx: TransactionRequest,
        tx_type: Option<TxType>,
    ) -> Result<bool> {
        let mut tx = tx
            .with_from(self.signer.address())
            .with_nonce(self.nonce_manager.get_next_nonce(self).await?)
            .with_chain_id(self.chain.id());

        let tx_type = tx_type.unwrap_or(TxType::Eip1559);

        match tx_type {
            TxType::Legacy => {
                let gas_price = self
                    .provider
                    .get_gas_price()
                    .await
                    .map_err(ClientError::Rpc)?;
                tx.set_gas_price(gas_price);
            }
            TxType::Eip1559 => {
                let fee = self
                    .provider
                    .estimate_eip1559_fees(None)
                    .await
                    .map_err(ClientError::Rpc)?;
                tx.set_max_fee_per_gas(fee.max_fee_per_gas);
                tx.set_max_priority_fee_per_gas(fee.max_priority_fee_per_gas);
            }
            _ => {
                return Err(crate::Error::EvmClient(ClientError::UnexpectedTxType(
                    tx_type,
                )));
            }
        };

        let gas = self
            .provider
            .estimate_gas(&tx)
            .await
            .map_err(ClientError::Rpc)?;
        tx.set_gas_limit(gas);

        let envelope = self.sign_tx_request(tx).await?;

        let receipt = self
            .provider
            .send_tx_envelope(envelope)
            .await
            .map_err(ClientError::Rpc)?
            .get_receipt()
            .await
            .map_err(ClientError::PendingTx)?;

        let (_, url) = self.chain.etherscan_urls().unwrap_or(("", ""));

        let status = receipt.status();
        let tx_hash = format!("{url}/tx/{}", receipt.transaction_hash());

        match status {
            true => {
                println!("Transaction successful: {tx_hash}")
            }
            false => {
                println!("Transaction failed: {tx_hash}")
            }
        }

        Ok(true)
    }

    /// Approves a spender to transfer tokens if needed.
    ///
    /// - Returns Ok(true) immediately if the token is native.
    /// - If ignore_allowance is true, skips the allowance check.
    /// - Sends an approval tx only if the current allowance is less than the requested amount.
    ///
    /// # Errors
    ///
    /// Fails if the allowance query or the transaction fails.
    #[tracing::instrument(skip_all)]
    pub async fn approve(
        &self,
        token: Token,
        spender: Address,
        amount: U256,
        ignore_allowance: bool,
    ) -> Result<bool> {
        if token.is_native() {
            return Ok(true);
        }

        let instance = IERC20::new(token.address(), &self.provider);

        let allowance = match ignore_allowance {
            true => U256::ZERO,
            false => {
                instance
                    .allowance(self.signer.address(), spender)
                    .call()
                    .await
                    .map_err(ClientError::Contract)?
                    ._0
            }
        };

        match allowance < amount {
            true => {
                let tx = TransactionRequest::default()
                    .with_input(IERC20::approveCall { spender, amount }.abi_encode())
                    .with_to(token.address());

                self.send_transaction(tx, None).await
            }
            false => Ok(true),
        }
    }

    pub async fn sign_message(&self, message: &String) -> Result<String> {
        let signature = self
            .signer
            .sign_message(message.as_bytes())
            .await
            .map_err(ClientError::Signer)?;
        let signature = encode_prefixed(signature.as_bytes());
        Ok(signature)
    }
}

pub trait ClientNonceManager<P: Provider>: Default {
    async fn get_next_nonce(&self, client: &Client<P, Self>) -> Result<u64>;
}

#[derive(Default)]
pub struct StrictNonceManager;

impl<P: Provider> ClientNonceManager<P> for StrictNonceManager {
    async fn get_next_nonce(&self, client: &Client<P, Self>) -> Result<u64> {
        let nonce = client
            .provider
            .get_transaction_count(client.signer.address())
            .await
            .map_err(ClientError::Rpc)?;
        Ok(nonce)
    }
}
