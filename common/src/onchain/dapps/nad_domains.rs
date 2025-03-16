use crate::{
    Result,
    onchain::{constants::MONAD_CHAIN_ID, error::ClientError},
};
use alloy::{
    network::{Ethereum, TransactionBuilder},
    primitives::{Address, FixedBytes, U256, address},
    providers::Provider,
    rpc::types::TransactionRequest,
    sol,
    sol_types::SolCall,
};
use fake::{Fake, faker::internet::en::Username};
use rquest::Client as RquestClient;
use serde::Deserialize;

use crate::onchain::client::Client as EvmClient;

#[derive(Debug, Deserialize)]
struct SignatureResponse {
    signature: String,
    nonce: U256,
    deadline: U256,
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

const NAD_DOMAINS_CA: Address = address!("0x758D80767a751fc1634f579D76e1CcaAb3485c9c");

fn get_valid_domain_name() -> String {
    let invalid_domain_name: String = Username().fake();
    invalid_domain_name.replace("_", "")
}

pub async fn mint<P>(evm_client: &EvmClient<P>, http_client: RquestClient) -> Result<bool>
where
    P: Provider<Ethereum>,
{
    let zero_bytes = FixedBytes::<32>::ZERO;

    let domain_name = get_valid_domain_name();
    let url = format!(
        "https://api.nad.domains/register/signature?name={0}&nameOwner={1}&setAsPrimaryName={2}&referrer={3}&discountKey={4}&discountClaimProof={5}&chainId={6}",
        domain_name,
        evm_client.signer.address(),
        true,
        Address::ZERO,
        zero_bytes,
        zero_bytes,
        MONAD_CHAIN_ID
    );

    let res = http_client.get(url).send().await?.json::<SignatureResponse>().await?;

    let tx = TransactionRequest::default()
        .with_input(
            INadDomains::registerWithSignatureCall {
                params: RegisterParams {
                    name: domain_name,
                    nameOwner: evm_client.signer.address(),
                    setAsPrimaryName: true,
                    referrer: Address::ZERO,
                    discountKey: FixedBytes::ZERO,
                    discountClaimProof: zero_bytes.into(),
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
