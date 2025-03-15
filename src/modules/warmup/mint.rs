use alloy::{network::Ethereum, providers::Provider};
use common::{
    onchain::{client::Client as EvmClient, dapps::nad_domains},
    state::Nft,
};
use database::entity::account::Model as AccountModel;

use crate::Result;

pub async fn mint<P>(
    platform: Nft,
    account: &AccountModel,
    evm_client: &EvmClient<P>,
) -> Result<bool>
where
    P: Provider<Ethereum>,
{
    let res = match platform {
        Nft::NadDomains => {
            let http_client = account.http_client()?;
            nad_domains::mint(evm_client, http_client).await?
        }
    };

    Ok(res)
}
