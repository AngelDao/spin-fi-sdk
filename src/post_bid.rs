use near_crypto::{InMemorySigner, KeyType, SecretKey};
use near_jsonrpc_client::auth::Unauthenticated;
use near_jsonrpc_client::JsonRpcClient;
use near_primitives::transaction::Transaction;
use near_primitives::transaction::{Action, FunctionCallAction};

#[path = "./utils/create_tx.rs"]
mod create_tx;
#[path = "./utils/send_tx.rs"]
mod send_tx;

pub async fn run(
    client: &JsonRpcClient<Unauthenticated>,
    signer: &InMemorySigner,
) -> Result<(), ()> {
    let actions: Vec<Action::FunctionCall::FunctionCallAction> =
        vec![Action::FunctionCall(FunctionCallAction {
            method_name: "bid".to_string(),
            args: json!({
                "market_id": 1,
                "price": "12",
                "quantity": "10",
                "ttl": 60
            })
            .to_string()
            .into_bytes(),
            gas: 100_000_000_000_000, // 100 TeraGas
            deposit: 0,
        })];
    let tx: Transaction = create_tx::run(client, signer, actions).expect("failed");
    send_tx::run(client, signer, tx);
}
