use crate::onchain::{client::EvmClient, dapps::hashflow::swap};
use alloy::{
    network::EthereumWallet, primitives::address, providers::ProviderBuilder,
    signers::local::PrivateKeySigner,
};
use rquest::{Client as RquestClient, Impersonate};
use std::{str::FromStr, sync::Arc};

mod onchain;

const PK: &str = "";

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let signer = PrivateKeySigner::from_str(PK).unwrap();
    let mut wallet = EthereumWallet::default();
    wallet.register_signer(signer.clone());
    let rpc_url = "https://testnet-rpc.monad.xyz".parse()?;
    let provider = Arc::new(ProviderBuilder::new().wallet(wallet).on_http(rpc_url));

    let evm_client = EvmClient::new(signer, Arc::clone(&provider));
    let token_in = address!("0x0000000000000000000000000000000000000000");
    let token_out = address!("0xf817257fed379853cde0fa4f97ab987181b1e5ea");
    let amount: u64 = 10000000000000000;

    let rquest_client = RquestClient::builder()
        .impersonate(Impersonate::Chrome133)
        .build()?;

    swap(&evm_client, rquest_client, token_in, token_out, amount).await?;
    Ok(())
}
