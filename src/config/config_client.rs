use crate::config::{connect_client, connect_wallet};
use crate::sdk;
use near_crypto::InMemorySigner;
use near_jsonrpc_client::auth::Unauthenticated;
use near_jsonrpc_client::{methods, JsonRpcClient};

pub fn initialize(
    rpc_client_url: &str,
    account_id: &str,
    account_private_accesskey: &str,
) -> sdk::SpinSDK {
    let signer: InMemorySigner =
        connect_wallet::run(account_id, account_private_accesskey).unwrap();
    let client = connect_client::run(rpc_client_url);
    sdk::SpinSDK { client, signer }
}
