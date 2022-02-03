use near_crypto::{InMemorySigner, KeyType, SecretKey};
use near_jsonrpc_client::auth::Unauthenticated;
use near_jsonrpc_client::JsonRpcClient;
use near_primitives::transaction::Transaction;
use near_primitives::transaction::{Action, FunctionCallAction, TransferAction};
use serde_json::json;

#[path = "./utils/create_tx.rs"]
mod create_tx;
#[path = "./utils/send_tx.rs"]
mod send_tx;

pub async fn run(
    client: &JsonRpcClient<Unauthenticated>,
    signer: &InMemorySigner,
) -> Result<(), ()> {
    let market_id: u8 = 1;
    let ttl: u8 = 60;
    let actions: Vec<Action> = vec![Action::FunctionCall(FunctionCallAction {
        method_name: "bid".to_string(),
        args: json!({
            "market_id": 1,
            "price": "12",
            "quantity": "1000000000000000000000000",
            "ttl": 60*60*24,
            "market_order": false,
        })
        .to_string()
        .into_bytes(),
        gas: 100_000_000_000_000, // 100 TeraGas
        deposit: 0,
    })];
    // send 1 yocto
    // let actions: Vec<Action> = vec![Action::Transfer(TransferAction { deposit: 1 })];
    println!("{:#?}", actions);
    let tx: Transaction = create_tx::run(client, signer, actions)
        .await
        .expect("failed");
    println!("{:#?}", tx);
    send_tx::run(client, signer, tx).await.expect("failed send");
    Ok(())
}
