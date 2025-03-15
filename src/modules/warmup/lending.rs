use std::sync::Arc;

use alloy::{network::Ethereum, primitives::U256, providers::Provider};
use common::{
    Error as CommonError,
    config::Config,
    onchain::{
        client::Client as EvmClient,
        dapps::{apriori, common::ONE_HUNDRED, kinza, shmonad},
        error::ClientError,
    },
    state::Lending,
    utils::random::random_in_range,
};

use crate::{Error, Result};

pub async fn deposit<P>(
    lending: Lending,
    evm_client: &EvmClient<P>,
    config: Arc<Config>,
) -> Result<bool>
where
    P: Provider<Ethereum>,
{
    let balance = evm_client
        .provider
        .get_balance(evm_client.address())
        .await
        .map_err(|e| Error::Common(CommonError::EvmClient(ClientError::Rpc(e))))?;
    let ratio = random_in_range(config.deposit_ratio);
    let amount_in = balance * (ONE_HUNDRED - U256::from(ratio) / ONE_HUNDRED);

    let res = match lending {
        Lending::Apriori => apriori::deposit(evm_client, amount_in).await?,
        Lending::Kinza => kinza::deposit(evm_client, amount_in).await?,
        Lending::Shmonad => shmonad::deposit(evm_client, amount_in).await?,
    };

    Ok(res)
}
