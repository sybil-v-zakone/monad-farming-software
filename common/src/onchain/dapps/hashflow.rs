use alloy::{
    hex::FromHex,
    network::Ethereum,
    primitives::{Address, FixedBytes, U256, address},
    providers::Provider,
    sol,
};
use rquest::{Client as RquestClient, header};
use serde::{Deserialize, Serialize};

use crate::{
    Result,
    onchain::{client::Client as EvmClient, error::ClientError, token::Token},
};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct Request {
    base_chain: RequestChain,
    quote_chain: RequestChain,
    source: &'static str,
    rfqs: Vec<RequestRfq>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct RequestChain {
    chain_id: u32,
    chain_type: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct RequestRfq {
    base_token: Address,
    quote_token: Address,
    base_token_amount: String,
    trader: Address,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Response {
    quotes: Vec<Quote>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Quote {
    quote_data: QuoteData,
    signature: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct QuoteData {
    quote_token_amount: String,
    pool: Address,
    txid: String,
    nonce: u64,
    quote_expiry: u64,
}

sol! {
    #[sol(rpc)]
    interface IHashflowRouter {
        /// @notice Executes an intra-chain RFQ-T trade.
        /// @param quote The quote data to be executed.
        function tradeRFQT(RFQTQuote memory quote) external payable;
    }

    struct RFQTQuote {
        /// @notice The address of the HashflowPool to trade against.
        address pool;
        /**
         * @notice The external account linked to the HashflowPool.
         * If the HashflowPool holds funds, this should be address(0).
         */
        address externalAccount;
        /// @notice The recipient of the quoteToken at the end of the trade.
        address trader;
        /**
         * @notice The account "effectively" making the trade (ultimately receiving the funds).
         * This is commonly used by aggregators, where a proxy contract (the 'trader')
         * receives the quoteToken, and the effective trader is the user initiating the call.
         *
         * This field DOES NOT influence movement of funds. However, it is used to check against
         * quote replay.
         */
        address effectiveTrader;
        /// @notice The token that the trader sells.
        address baseToken;
        /// @notice The token that the trader buys.
        address quoteToken;
        /**
         * @notice The amount of baseToken sold in this trade. The exchange rate
         * is going to be preserved as the quoteTokenAmount / baseTokenAmount ratio.
         *
         * Most commonly, effectiveBaseTokenAmount will == baseTokenAmount.
         */
        uint256 effectiveBaseTokenAmount;
        /// @notice The max amount of baseToken sold.
        uint256 baseTokenAmount;
        /// @notice The amount of quoteToken bought when baseTokenAmount is sold.
        uint256 quoteTokenAmount;
        /// @notice The Unix timestamp (in seconds) when the quote expires.
        /// @dev This gets checked against block.timestamp.
        uint256 quoteExpiry;
        /// @notice The nonce used by this effectiveTrader. Nonces are used to protect against replay.
        uint256 nonce;
        /// @notice Unique identifier for the quote.
        /// @dev Generated off-chain via a distributed UUID generator.
        bytes32 txid;
        /// @notice Signature provided by the market maker (EIP-191).
        bytes signature;
    }
}

const HASHFLOW_CONTRACT_ADDRESS: Address = address!("0xca310b1b942a30ff4b40a5e1b69ab4607ec79bc1");

async fn get_quote(
    rquest_client: RquestClient,
    token_in: Address,
    token_out: Address,
    amount_in: U256,
    trader: Address,
) -> Result<Quote> {
    let mut headers = header::HeaderMap::new();
    headers.insert("content-type", "application/json".parse().unwrap());

    let request_chain = RequestChain { chain_type: "evm".to_string(), chain_id: 10143 };

    let req = Request {
        base_chain: request_chain.clone(),
        quote_chain: request_chain,
        source: "hashflow",
        rfqs: vec![RequestRfq {
            base_token: token_in,
            quote_token: token_out,
            base_token_amount: amount_in.to_string(),
            trader,
        }],
    };

    let res = rquest_client
        .post("https://api.hashflow.com/client/v3/rfq")
        .headers(headers)
        .json(&req)
        .send()
        .await?
        .json::<Response>()
        .await?;

    Ok(res.quotes.into_iter().next().unwrap())
}

impl RFQTQuote {
    pub fn new_from_quote(
        quote: Quote,
        trader: Address,
        token_in: Token,
        token_out: Token,
        amount: U256,
    ) -> Result<Self> {
        Ok(Self {
            pool: quote.quote_data.pool,
            externalAccount: Address::ZERO,
            trader,
            effectiveTrader: trader,
            baseToken: token_in.address(),
            quoteToken: token_out.address(),
            effectiveBaseTokenAmount: amount,
            baseTokenAmount: amount,
            quoteTokenAmount: U256::from_str_radix(&quote.quote_data.quote_token_amount, 10)
                .map_err(ClientError::Parse)?,
            quoteExpiry: U256::from(quote.quote_data.quote_expiry),
            nonce: U256::from(quote.quote_data.nonce),
            txid: FixedBytes::from_hex(quote.quote_data.txid).map_err(ClientError::FromHex)?,
            signature: alloy::hex::decode(quote.signature).map_err(ClientError::FromHex)?.into(),
        })
    }
}

pub async fn swap<P>(
    evm_client: &EvmClient<P>,
    rquest_client: RquestClient,
    token_in: Token,
    token_out: Token,
    amount_in: U256,
) -> Result<bool>
where
    P: Provider<Ethereum>,
{
    let quote = get_quote(
        rquest_client,
        token_in.address(),
        token_out.address(),
        amount_in,
        evm_client.signer.address(),
    )
    .await?;

    let contract = IHashflowRouter::new(HASHFLOW_CONTRACT_ADDRESS, &evm_client.provider);

    let tx_req = contract
        .tradeRFQT(RFQTQuote::new_from_quote(
            quote,
            evm_client.signer.address(),
            token_in,
            token_out,
            amount_in,
        )?)
        .value(amount_in)
        .into_transaction_request();

    evm_client.send_transaction(tx_req, None).await
}
