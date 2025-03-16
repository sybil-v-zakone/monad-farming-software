use std::sync::Arc;

use alloy::{network::Ethereum, primitives::U256, providers::Provider};
use common::{
    config::Config,
    onchain::{
        client::Client as EvmClient,
        dapps::{ambient, bean, common::ONE_HUNDRED, hashflow},
        token::Token,
    },
    state::Dex,
    utils::random::random_in_range,
};
use database::entity::impls::prelude::AccountModel;
use rand::seq::IndexedRandom;

use crate::Result;

use super::error::WarmupError;

pub async fn swap<P>(
    dex: Dex,
    account: &AccountModel,
    evm_client: &EvmClient<P>,
    config: Arc<Config>,
) -> Result<bool>
where
    P: Provider<Ethereum>,
{
    let nonzero_tokens = evm_client.get_nonzero_token_balances().await?;

    let (token_in, token_in_balance) = nonzero_tokens
        .choose(&mut rand::rng())
        .ok_or_else(|| WarmupError::EmptyWallet(evm_client.signer.address()))?;

    let token_out = Token::random_excluding(*token_in);
    let ratio = random_in_range(config.swap_ratio);
    let amount_in = token_in_balance * U256::from(ratio) / ONE_HUNDRED;

    let res = match dex {
        Dex::Hashflow => {
            let http_client = account.http_client()?;
            hashflow::swap(evm_client, http_client, *token_in, token_out, amount_in).await?
        }
        Dex::Ambient => ambient::swap(evm_client, amount_in, *token_in, token_out).await?,
        Dex::Bean => bean::swap(evm_client, amount_in, *token_in, token_out).await?,
    };

    Ok(res)
}
