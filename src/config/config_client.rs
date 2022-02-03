use near_crypto::InMemorySigner;
use near_jsonrpc_client::auth::Unauthenticated;
use near_jsonrpc_client::{methods, JsonRpcClient};

#[path = "./connect_client.rs"]
mod connect_client;
#[path = "./connect_wallet.rs"]
mod connect_wallet;
#[path = "./utils/sdk_struct.rs"]
pub mod sdk_struct;

pub fn initialize(
    rpc_client_url: &str,
    account_id: &str,
    account_private_accesskey: &str,
) -> sdk_struct::SpinSDK {
    let signer: InMemorySigner =
        connect_wallet::run(account_id, account_private_accesskey).unwrap();
    let client = connect_client::run(rpc_client_url);
    sdk_struct::SpinSDK { client, signer }
}
