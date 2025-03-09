use alloy::{
    hex::{self, FromHex},
    network::Network,
    primitives::{Address, FixedBytes, U256, address},
    providers::{Provider, fillers::TxFiller},
    rpc::types::TransactionRequest,
    sol,
};
use rquest::{Client as RquestClient, header};
use serde::{Deserialize, Serialize};

use crate::onchain::{client::EvmClient, error::Result};

#[derive(Serialize)]
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
    chain_type: String,
    chain_id: u32,
}

#[derive(Serialize)]
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
    base_token: Address,
    quote_token: Address,
    base_token_amount: String,
    quote_token_amount: String,
    trader: Address,
    effective_trader: Address,
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
    amount: u64,
    trader: Address,
) -> Result<Quote> {
    let mut headers = header::HeaderMap::new();
    headers.insert("content-type", "application/json".parse().unwrap());

    let request_chain = RequestChain {
        chain_type: "evm".to_string(),
        chain_id: 10143,
    };

    let req = Request {
        base_chain: request_chain.clone(),
        quote_chain: request_chain,
        source: "hashflow",
        rfqs: vec![RequestRfq {
            base_token: token_in,
            quote_token: token_out,
            base_token_amount: amount.to_string(),
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

    Ok(res.quotes[0].clone())
}

pub async fn swap<F, P, N>(
    evm_client: &EvmClient<F, P, N>,
    rquest_client: RquestClient,
    token_in: Address,
    token_out: Address,
    amount: u64,
) -> Result<bool>
where
    F: TxFiller<N>,
    P: Provider<N>,
    N: Network<TransactionRequest = TransactionRequest>,
{
    let quote = get_quote(
        rquest_client,
        token_in,
        token_out,
        amount,
        evm_client.signer.address(),
    )
    .await?;

    let contract = IHashflowRouter::new(HASHFLOW_CONTRACT_ADDRESS, &evm_client.provider);

    println!("{:?\n\n}", quote.quote_data);

    let tx_req = contract
        .tradeRFQT(RFQTQuote::new_from_quote(
            quote,
            evm_client.signer.address(),
            token_in,
            token_out,
            U256::from(amount),
        )?)
        .value(U256::from(amount))
        .into_transaction_request();

    print!("{:?}", tx_req);

    let status = evm_client.send_transaction(tx_req).await?;

    Ok(true)
}

impl RFQTQuote {
    pub fn new_from_quote(
        quote: Quote,
        trader: Address,
        token_in: Address,
        token_out: Address,
        amount: U256,
    ) -> Result<Self> {
        Ok(Self {
            pool: quote.quote_data.pool,
            externalAccount: Address::ZERO,
            trader,
            effectiveTrader: trader,
            baseToken: token_in,
            quoteToken: token_out,
            effectiveBaseTokenAmount: amount,
            baseTokenAmount: amount,
            quoteTokenAmount: U256::from_str_radix(&quote.quote_data.quote_token_amount, 10)?,
            quoteExpiry: U256::from(quote.quote_data.quote_expiry),
            nonce: U256::from(quote.quote_data.nonce),
            txid: FixedBytes::from_hex(quote.quote_data.txid)?,
            signature: alloy::hex::decode(quote.signature)?.into(),
        })
    }
}

// TODO: ERROR reverted

// TransactionRequest { from: None, to: Some(Call(0xca310b1b942a30ff4b40a5e1b69ab4607ec79bc1)), gas_price: None, max_fee_per_gas: None, max_priority_fee_per_gas: None, max_fee_per_blob_gas: None, gas: None, value: Some(1000000000), input: TransactionInput { input: Some(0x30e08c870000000000000000000000000cf6c9089e8b8eafcbfbbb430b55a3904c7402260000000000000000000000000000000000000000000000000000000000000000000000000000000000000000e6b3e9aa99c4c01b041779ef7acf659ab4655a50000000000000000000000000e6b3e9aa99c4c01b041779ef7acf659ab4655a500000000000000000000000000000000000000000000000000000000000000000000000000000000000000000f817257fed379853cde0fa4f97ab987181b1e5ea000000000000000000000000000000000000000000000000000000003b9aca00000000000000000000000000000000000000000000000000000000003b9aca0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000067cc4dd800000000000000000000000000000000000000000000000000000195760f29911000000c8000c800000017254c3880ffffffffffffff002617a841972bd1000000000000000000000000000000000000000000000000000000000000000001a000000000000000000000000000000000000000000000000000000000000000411e6f7d4afe2a4870e0df6e24c0ca1993e38b4cf0e1f3bff69b48f276fb53427f559a59588cf3c96eb907d27dea6970dd7e38a31b602be0195b91cccbf7ce8ec91b00000000000000000000000000000000000000000000000000000000000000Error: Rpc(ErrorResp(ErrorPayload { code: -32603, message: "execution reverted", data: Some(RawValue("0x")) }))
// ), data: None }, nonce: None, chain_id: None, access_list: None, transaction_type: None, blob_versioned_hashes: None, sidecar: None, authorization_list: None }
