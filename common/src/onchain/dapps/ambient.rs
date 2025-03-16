use alloy::{
    network::TransactionBuilder,
    primitives::{Address, U256, address},
    rpc::types::TransactionRequest,
};

use alloy::{
    network::Ethereum,
    providers::Provider,
    sol,
    sol_types::{SolCall, SolValue},
};
use thiserror::Error;

use crate::{
    Result,
    onchain::{client::Client, error::ClientError, token::Token},
};

const IMPACT_CA: Address = address!("0x70a6a0C905af5737aD73Ceba4e6158e995031d4B");
const DEX_CA: Address = address!("0x88B96aF200c8a9c35442C8AC6cd3D22695AaE4F0");
const POOL_IDX: U256 = U256::from_limbs([36000, 0, 0, 0]);
const POOL_TIP: u16 = 0;

const MAX_PRICE: u128 = 21267430153580247136652501917186561137;
const MIN_PRICE: u128 = 65537;
const SETTLE_FLAGS: u8 = 0;

const CALL_PATH: u16 = 1;

enum Pool {
    MonUsdc,
    MonShmon,
    UsdcShmon,
}

#[derive(Error, Debug)]
pub enum AmbientError {
    #[error("no pools for `{0}` and `{1}` available")]
    UnsupportedTokens(Token, Token),
}

macro_rules! pool_match {
    ($a:expr, $b:expr; $( ($t1:ident, $t2:ident) => $pool:ident ),* $(,)?) => {
        match ($a, $b) {
            $(
                (Token::$t1, Token::$t2) | (Token::$t2, Token::$t1) => Some(Pool::$pool),
            )*
            _ => None,
        }
    };
}

impl Pool {
    const fn quote(&self) -> Token {
        match self {
            Pool::MonUsdc => Token::USDC,
            Pool::MonShmon => Token::SHMON,
            Pool::UsdcShmon => Token::USDC,
        }
    }

    const fn base(&self) -> Token {
        match self {
            Pool::MonUsdc => Token::MON,
            Pool::MonShmon => Token::MON,
            Pool::UsdcShmon => Token::SHMON,
        }
    }

    fn from_tokens(token_a: Token, token_b: Token) -> Option<Self> {
        pool_match! { token_a, token_b;
            (MON, USDC) => MonUsdc,
            (MON, SHMON) => MonShmon,
            (USDC, SHMON) => UsdcShmon,
        }
    }
}

sol! {
    #[sol(rpc)]
    interface IDex {
        function userCmd(uint16 callpath, bytes calldata cmd) external payable returns (bytes memory);
    }

    #[allow(clippy::too_many_arguments)]
    #[sol(rpc)]
    contract Impact {
        function calcImpact(
            address base,
            address quote,
            uint256 poolIdx,
            bool isBuy,
            bool inBaseQty,
            uint128 qty,
            uint16 poolTip,
            uint128 limitPrice
        ) external view returns (int128 baseFlow, int128 quoteFlow, uint128 finalPrice);
    }

    struct UserCmd {
        address base;
        address quote;
        uint256 poolIdx;
        bool isBuy;
        bool inBaseQty;
        uint128 qty;
        uint16 tip;
        uint128 limitPrice;
        uint128 minOut;
        uint8 settleFlags;
    }
}

async fn get_amount_out<P>(
    client: &Client<P>,
    base_token: Token,
    quote_token: Token,
    is_buy: bool,
    in_base_qty: bool,
    qty: u128,
    limit_price: u128,
) -> Result<i128>
where
    P: Provider<Ethereum>,
{
    let impact_instance = Impact::new(IMPACT_CA, &client.provider);

    let out = impact_instance
        .calcImpact(
            base_token.address(),
            quote_token.address(),
            POOL_IDX,
            is_buy,
            in_base_qty,
            qty,
            POOL_TIP,
            limit_price,
        )
        .call()
        .await
        .map_err(ClientError::Contract)?;

    let amount_out = match is_buy {
        true => out.quoteFlow,
        false => out.baseFlow,
    };

    Ok(amount_out)
}

async fn build_cmd_data<P>(
    client: &Client<P>,
    token_in: &Token,
    pool: Pool,
    amount: U256,
) -> Result<Vec<u8>>
where
    P: Provider<Ethereum>,
{
    let (is_buy, in_base_qty, limit_price) =
        if *token_in == Token::MON || token_in.address() == pool.base().address() {
            (true, true, MAX_PRICE)
        } else {
            (false, false, MIN_PRICE)
        };

    let qty = amount.try_into()?;

    let amount_out =
        get_amount_out(client, pool.base(), pool.quote(), is_buy, in_base_qty, qty, limit_price)
            .await?
            .unsigned_abs();

    let amount_out = amount_out * (100u128 - 1u128) / 100u128;

    let cmd = UserCmd {
        base: pool.base().address(),
        quote: pool.quote().address(),
        poolIdx: POOL_IDX,
        isBuy: is_buy,
        inBaseQty: in_base_qty,
        qty,
        tip: POOL_TIP,
        limitPrice: limit_price,
        minOut: amount_out,
        settleFlags: SETTLE_FLAGS,
    }
    .abi_encode();

    Ok(cmd)
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
    let approved = client.approve(token_in, DEX_CA, amount_in, false).await?;

    if !approved {
        return Ok(false);
    }

    let pool = Pool::from_tokens(token_in, token_out)
        .ok_or(AmbientError::UnsupportedTokens(token_in, token_out))?;
    let cmd = build_cmd_data(client, &token_in, pool, amount_in).await?;
    let value = match token_in.is_native() {
        true => amount_in,
        false => U256::ZERO,
    };

    let tx = TransactionRequest::default()
        .with_input(IDex::userCmdCall { callpath: CALL_PATH, cmd: cmd.into() }.abi_encode())
        .with_to(DEX_CA)
        .with_value(value);

    client.send_transaction(tx, None).await
}
