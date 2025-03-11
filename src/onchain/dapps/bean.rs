use crate::{
    Result,
    onchain::{client::Client, token::Token},
};
use alloy::{
    network::{Ethereum, TransactionBuilder},
    primitives::{Address, U256, address},
    providers::Provider,
    rpc::types::TransactionRequest,
    sol,
    sol_types::SolCall,
};
use chrono::{Duration, Utc};
use thiserror::Error;

use super::common::{ONE_HUNDRED, SLIPPAGE};

sol! {
    #[sol(rpc)]
    interface IBeanRouter {
        function swapExactETHForTokens(
            uint256 amountOutMin,
            address[] calldata path,
            address to,
            uint256 deadline
        ) external payable returns (uint256[] memory amounts);

        function swapExactTokensForETH(
            uint256 amountIn,
            uint256 amountOutMin,
            address[] calldata path,
            address to,
            uint256 deadline
        ) external returns (uint256[] memory amounts);

        function swapExactTokensForTokens(
            uint256 amountIn,
            uint256 amountOutMin,
            address[] calldata path,
            address to,
            uint256 deadline
        ) external returns (uint256[] memory amounts);

        function getAmountsOut(uint256 amountIn, address[] calldata path) external view returns (uint256[] memory amounts);
    }
}

#[derive(Error, Debug)]
pub enum BeanError {
    #[error("amount out is missing")]
    AmountOutMissing,
}

const BEAN_ROUTER: Address = address!("0xCa810D095e90Daae6e867c19DF6D9A8C56db2c89");

async fn get_amount_out<P>(
    client: &Client<P>,
    amount_in: U256,
    token_in: &Token,
    token_out: &Token,
) -> Result<U256>
where
    P: Provider<Ethereum>,
{
    let router = IBeanRouter::new(BEAN_ROUTER, &client.provider);

    let amount_out = router
        .getAmountsOut(amount_in, vec![token_in.address(), token_out.address()])
        .call()
        .await
        .map_err(crate::onchain::error::ClientError::Contract)?
        .amounts
        .into_iter()
        .nth(1)
        .ok_or(BeanError::AmountOutMissing)?;

    Ok(amount_out * (ONE_HUNDRED - SLIPPAGE) / ONE_HUNDRED)
}

fn get_deadline() -> U256 {
    let deadline = Utc::now() + Duration::minutes(20);
    U256::from(deadline.timestamp())
}

// token_in will always be Wrapped MON
async fn swap_exact_eth_for_tokens<P>(
    client: &Client<P>,
    amount_in: U256,
    amount_out: U256,
    token_out: Token,
) -> Result<bool>
where
    P: Provider<Ethereum>,
{
    let tx = TransactionRequest::default()
        .with_input(
            IBeanRouter::swapExactETHForTokensCall {
                amountOutMin: amount_out,
                path: vec![Token::WMON.address(), token_out.address()],
                to: client.signer.address(),
                deadline: get_deadline(),
            }
            .abi_encode(),
        )
        .with_to(BEAN_ROUTER)
        .with_value(amount_in);

    client.send_transaction(tx, None).await
}

// token_out will always be Wrapped MON
async fn swap_exact_tokens_for_eth<P>(
    client: &Client<P>,
    amount_in: U256,
    amount_out: U256,
    token_in: Token,
) -> Result<bool>
where
    P: Provider<Ethereum>,
{
    let tx = TransactionRequest::default()
        .with_input(
            IBeanRouter::swapExactTokensForETHCall {
                amountIn: amount_in,
                amountOutMin: amount_out,
                path: vec![token_in.address(), Token::WMON.address()],
                to: client.signer.address(),
                deadline: get_deadline(),
            }
            .abi_encode(),
        )
        .with_to(BEAN_ROUTER);

    client.send_transaction(tx, None).await
}

async fn swap_exact_tokens_for_tokens<P>(
    client: &Client<P>,
    amount_in: U256,
    amount_out: U256,
    token_in: Token,
    token_out: Token,
) -> Result<bool>
where
    P: Provider<Ethereum>,
{
    let tx = TransactionRequest::default()
        .with_input(
            IBeanRouter::swapExactTokensForTokensCall {
                amountIn: amount_in,
                amountOutMin: amount_out,
                path: vec![token_in.address(), token_out.address()],
                to: client.signer.address(),
                deadline: get_deadline(),
            }
            .abi_encode(),
        )
        .with_to(BEAN_ROUTER);

    client.send_transaction(tx, None).await
}

pub async fn swap<P>(
    client: &Client<P>,
    amount_in: U256,
    token_in: Token,
    token_out: Token,
) -> Result<bool>
where
    P: Provider<Ethereum>,
{
    let approved = match token_in.is_native() {
        false => {
            client
                .approve(token_in, BEAN_ROUTER, amount_in, false)
                .await?
        }
        true => true,
    };

    if !approved {
        return Ok(false);
    }

    let (src, dst, case) = match (token_in, token_out) {
        (_, Token::MON) => (token_in, Token::WMON, 1),
        (Token::MON, _) => (Token::WMON, token_out, 2),
        _ => (token_in, token_out, 0),
    };

    let amount_out = get_amount_out(client, amount_in, &src, &dst).await?;

    match case {
        1 => swap_exact_tokens_for_eth(client, amount_in, amount_out, token_in).await,
        2 => swap_exact_eth_for_tokens(client, amount_in, amount_out, token_out).await,
        _ => swap_exact_tokens_for_tokens(client, amount_in, amount_out, token_in, token_out).await,
    }
}
