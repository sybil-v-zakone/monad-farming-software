use std::sync::Arc;

use alloy::{
    network::Ethereum,
    primitives::{
        U256,
        utils::{format_units, parse_ether},
    },
    providers::Provider,
};
use common::{
    config::Config,
    onchain::{client::Client as EvmClient, dapps::gas_zip, token::Token},
    utils::random::random_in_range,
};

use crate::Result;

use super::error::WarmupError;

fn get_bridge_amount(bridge_amount_range: [f64; 2]) -> U256 {
    let random_amount = random_in_range(bridge_amount_range);
    parse_ether(&random_amount.to_string())
        .expect("Check your \"bridge_amount_range\" in config.toml")
}

pub async fn bridge<P>(evm_client: &EvmClient<P>, config: Arc<Config>) -> Result<bool>
where
    P: Provider<Ethereum>,
{
    let native_balance = evm_client.get_native_balance().await?;
    let bridge_amount = get_bridge_amount(config.bridge_amount_range);

    if native_balance < bridge_amount {
        return Ok(false);
    }

    tracing::info!(
        "Gaszip | bridging {} {}",
        format_units(bridge_amount, Token::ETH.decimals()).map_err(WarmupError::FormatUnits)?,
        Token::ETH
    );

    let res = gas_zip::bridge(evm_client, bridge_amount).await?;

    Ok(res)
}
