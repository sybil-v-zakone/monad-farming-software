use crate::{Result, onchain::error::ClientError};
use alloy::{
    hex::FromHex,
    network::{Ethereum, TransactionBuilder},
    primitives::{Address, FixedBytes, U256, address},
    providers::Provider,
    rpc::types::TransactionRequest,
    sol,
    sol_types::SolCall,
};
use rquest::Client as RquestClient;
use serde::{Deserialize, Serialize};

use crate::onchain::client::Client as EvmClient;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct SignatureRequest<'a> {
    name: String,
    name_owner: Address,
    set_as_primary_name: bool,
    referrer: Address,
    discount_key: &'a str,
    discount_claim_proof: &'a str,
    chain_id: u32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SignatureResponse {
    signature: String,
    nonce: U256,
    deadline: u64,
}

sol! {
    struct RegisterParams {
        string name;
        address nameOwner;
        bool setAsPrimaryName;
        address referrer;
        bytes32 discountKey;
        bytes discountClaimProof;
        uint256 nonce;
        uint256 deadline;
    }

    interface INadDomains {
        function registerWithSignature(
            RegisterParams calldata params,
            bytes calldata signature
        ) external payable returns (uint256);
    }
}

const NAD_DOMAINS_SIGNATURE_URL: &'static str = "https://api.nad.domains/register/signature";
const DISCOUNT_KEY: &'static str =
    "0x0000000000000000000000000000000000000000000000000000000000000000";
const DISCOUNT_CLAIM_PROOF: &'static str =
    "0x0000000000000000000000000000000000000000000000000000000000000000";
const NAD_DOMAINS_CA: Address = address!("0x758D80767a751fc1634f579D76e1CcaAb3485c9c");

pub async fn mint<P>(evm_client: &EvmClient<P>, http_client: RquestClient) -> Result<bool>
where
    P: Provider<Ethereum>,
{
    // TODO: "Написать получение рандомного доменного имени"
    let domain_name: String = "sjdfksjkdf".to_string();

    let req = SignatureRequest {
        name: domain_name,
        name_owner: evm_client.signer.address(),
        set_as_primary_name: true,
        referrer: Address::ZERO,
        discount_key: DISCOUNT_KEY,
        discount_claim_proof: DISCOUNT_CLAIM_PROOF,
        chain_id: 10143,
    };

    let res = http_client
        .get(NAD_DOMAINS_SIGNATURE_URL)
        .json(&req)
        .send()
        .await?
        .json::<SignatureResponse>()
        .await?;

    let tx = TransactionRequest::default()
        .with_input(
            INadDomains::registerWithSignatureCall {
                params: RegisterParams {
                    name: req.name,
                    nameOwner: evm_client.signer.address(),
                    setAsPrimaryName: true,
                    referrer: Address::ZERO,
                    discountKey: FixedBytes::from_hex(DISCOUNT_KEY)
                        .map_err(ClientError::FromHex)?,
                    discountClaimProof: alloy::hex::decode(DISCOUNT_CLAIM_PROOF)
                        .map_err(ClientError::FromHex)?
                        .into(),
                    nonce: res.nonce,
                    deadline: U256::from(res.deadline),
                },
                signature: alloy::hex::decode(res.signature).map_err(ClientError::FromHex)?.into(),
            }
            .abi_encode(),
        )
        .with_to(NAD_DOMAINS_CA)
        .with_value(U256::from(20000000000000000u64));

    evm_client.send_transaction(tx, None).await
}
