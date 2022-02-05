use crate::utils::{create_tx, send_tx, structs::PlacedOrder};
use near_crypto::{InMemorySigner, KeyType, SecretKey};
use near_jsonrpc_client::auth::Unauthenticated;
use near_jsonrpc_client::JsonRpcClient;
use near_primitives::transaction::Transaction;
use near_primitives::transaction::{Action, FunctionCallAction, TransferAction};
use serde_json::json;

pub async fn run(
    client: &JsonRpcClient<Unauthenticated>,
    signer: &InMemorySigner,
    market_id: u8,
    order_id: u64,
) -> Result<(), ()> {
    let market_id: u8 = 1;
    let ttl: u8 = 60;
    let actions: Vec<Action> = vec![Action::FunctionCall(FunctionCallAction {
        method_name: "drop_order".to_string(),
        args: json!({
            "market_id": market_id,
            "order_id": order_id
        })
        .to_string()
        .into_bytes(),
        gas: 100_000_000_000_000, // 100 TeraGas
        deposit: 0,
    })];
    // send 1 yocto
    // let actions: Vec<Action> = vec![Action::Transfer(TransferAction { deposit: 1 })];
    println!("{:#?}", actions);
    let tx: Transaction = create_tx::run(client, signer, actions, "app_2.spin_swap.testnet")
        .await
        .expect("failed");
    println!("{:#?}", tx);
    send_tx::run(client, signer, tx).await.expect("failed send");
    Ok(())
}
