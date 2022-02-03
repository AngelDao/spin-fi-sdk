use near_jsonrpc_client::auth::Unauthenticated;
use near_jsonrpc_client::JsonRpcClient;

pub fn run(node: &str) -> JsonRpcClient<Unauthenticated> {
    let testnet_client = JsonRpcClient::connect(node);
    return testnet_client;
}
