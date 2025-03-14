use crate::{
    Result,
    onchain::{client::Client as EvmClient, token::Token},
};
use alloy::{
    network::{Ethereum, TransactionBuilder},
    primitives::U256,
    providers::Provider,
    rpc::types::TransactionRequest,
    sol,
    sol_types::SolCall,
};

sol! {
    interface IApriori {
        function deposit(uint256 assets, address receiver) external payable returns (uint256);
    }
}

pub async fn deposit<P>(evm_client: &EvmClient<P>, amount: U256) -> Result<bool>
where
    P: Provider<Ethereum>,
{
    let tx = TransactionRequest::default()
        .with_input(
            IApriori::depositCall { assets: amount, receiver: evm_client.signer.address() }
                .abi_encode(),
        )
        .with_to(Token::APRMON.address())
        .with_value(amount);

    evm_client.send_transaction(tx, None).await
}
