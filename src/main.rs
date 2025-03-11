use crate::onchain::client::Client as EvmClient;
use alloy::{
    network::Ethereum,
    primitives::{U256, address},
    providers::ProviderBuilder,
    signers::local::PrivateKeySigner,
};
use alloy_chains::NamedChain;
use cli::parse_cli_args;
use logger::init_logging;
use onchain::{client::StrictNonceManager, dapps::hashflow::swap, token::Token};
use rquest::{Client as RquestClient, Impersonate};
use std::str::FromStr;
use std::sync::Arc;

mod cli;
mod error;
mod logger;
mod onchain;

pub use crate::error::{Error, Result};

const PK: &str = "";

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let cli = parse_cli_args();
    init_logging(&cli.log_level);

    let signer = PrivateKeySigner::from_str(PK).unwrap();

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

    let token_in = Token::MON.address();
    let token_out = address!("0xf817257fed379853cde0fa4f97ab987181b1e5ea");

    let ZERO_POINT_ONE_MON = U256::from(100000000000000000u64);
    let ONE_USDC = U256::from(1000000u64);

    let rquest_client = RquestClient::builder()
        .impersonate(Impersonate::Chrome133)
        .build()?;

    swap(
        &evm_client,
        rquest_client,
        token_in,
        token_out,
        ZERO_POINT_ONE_MON,
    )
    .await?;

    Ok(())
}
