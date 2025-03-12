use crate::{
    Result,
    onchain::{client::Client as EvmClient, token::Token},
};
use alloy::{
    network::{Ethereum, TransactionBuilder},
    primitives::{Address, U256, address},
    providers::Provider,
    rpc::types::TransactionRequest,
    sol,
    sol_types::SolCall,
};

sol! {
    interface IKinza {
        function depositETH(address token, address receiver, uint16 referralCode) external payable returns (uint256);
    }
}

const KINZA_CA: Address = address!("0x21d6192677f4bbff6BCCF11FC7D5c3076bFF6F1B");

pub async fn deposit<P>(evm_client: &EvmClient<P>, amount: U256) -> Result<bool>
where
    P: Provider<Ethereum>,
{
    let tx = TransactionRequest::default()
        .with_input(
            IKinza::depositETHCall {
                token: Token::WMON.address(),
                receiver: evm_client.signer.address(),
                referralCode: 0,
            }
            .abi_encode(),
        )
        .with_to(KINZA_CA)
        .with_value(amount);

    evm_client.send_transaction(tx, None).await
}
