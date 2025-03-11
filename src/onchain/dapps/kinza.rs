use crate::{Result, onchain::client::Client as EvmClient};
use alloy::{
    network::{Ethereum, TransactionBuilder},
    primitives::{Address, U256, address},
    providers::Provider,
    rpc::types::TransactionRequest,
};

const KINZA_CA: Address = address!("0x21d6192677f4bbff6BCCF11FC7D5c3076bFF6F1B");

pub async fn deposit<P>(evm_client: &EvmClient<P>, amount: U256) -> Result<bool>
where
    P: Provider<Ethereum>,
{
    let address: String = evm_client.signer.address().to_string()[2..].to_string();

    let tx = TransactionRequest::default()
        .with_input(
            alloy::hex::decode(format!(
                "0x474cf53d000000000000000000000000760afe86e5de5fa0ee542fc7b7b713e1c5425701000000000000000000000000{}0000000000000000000000000000000000000000000000000000000000000000",
                address
            ))
            .unwrap()
        )
        .with_to(KINZA_CA)
        .with_value(amount);

    evm_client.send_transaction(tx, None).await
}
