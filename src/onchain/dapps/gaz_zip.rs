use crate::onchain::{client::EvmClient, error::OnchainResult};
use alloy::{
    network::{Network, TransactionBuilder},
    primitives::{U256, address},
    providers::{Provider, fillers::TxFiller},
    rpc::types::TransactionRequest,
};

pub async fn bridge<F, P, N>(evm_client: &EvmClient<F, P, N>, amount: u64) -> OnchainResult<bool>
where
    F: TxFiller<N>,
    P: Provider<N>,
    N: Network<TransactionRequest = TransactionRequest>,
{
    let call_data = String::from("0x0101b1");
    let call_data = alloy::hex::decode(call_data)?;
    println!("call_data: {:?}", call_data);

    let tx_req: TransactionRequest = TransactionRequest::default()
        .with_input(call_data)
        .to(address!("0x391E7C679d29bD940d63be94AD22A25d25b5A604"))
        .with_value(U256::from(amount));

    let status = evm_client.send_transaction(tx_req).await?;

    Ok(status)
}
