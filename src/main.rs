#![allow(unused, dead_code)]

use crate::onchain::client::Client as EvmClient;
use alloy::{
    network::{Ethereum, EthereumWallet},
    primitives::{Address, U16, U256, address},
    providers::ProviderBuilder,
    signers::local::PrivateKeySigner,
};
use alloy_chains::NamedChain;
use cli::parse_cli_args;
use logger::init_logging;
use onchain::{
    client::StrictNonceManager,
    dapps::{ambient, bean, shmonad},
    token::Token,
};
use rquest::{Client as RquestClient, Impersonate};
use std::{str::FromStr, sync::Arc};

mod cli;
mod error;
mod logger;
mod onchain;

pub use crate::error::{Error, Result};

const POOL_IDX: U256 = U256::from_limbs([36000, 0, 0, 0]);

const MAX_PRICE: U256 = U256::from_limbs([0x91d9f90d93d6b061, 0x100d73bf4fae6d4c, 0, 0]);

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let cli = parse_cli_args();
    init_logging(&cli.log_level);

    let signer = PrivateKeySigner::random();

    let rpc_url = "https://testnet-rpc.monad.xyz".parse()?;
    let provider = Arc::new(
        ProviderBuilder::new()
            .network::<Ethereum>()
            .disable_recommended_fillers()
            .on_http(rpc_url),
    );

    let evm_client = EvmClient::<_, StrictNonceManager>::new(
        signer,
        NamedChain::MonadTestnet.into(),
        Arc::clone(&provider),
    );
    // let token_in = Address::ZERO;
    let wmon = address!("0x760AfE86e5de5fa0Ee542fc7B7B713e1c5425701");

    let token_out = address!("0xf817257fed379853cde0fa4f97ab987181b1e5ea");
    let ONE_MON = U256::from(1000000000000000000u64);
    let ONE_USDC = U256::from(1000000u64);

    // let rquest_client = RquestClient::builder()
    //     .impersonate(Impersonate::Chrome133)
    //     .build()?;

    Ok(())
}
