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
    order: PlacedOrder,
    decimals: u8,
) -> Result<(), ()> {
    let actions: Vec<Action> = vec![Action::FunctionCall(FunctionCallAction {
        method_name: "ask".to_string(),
        args: json!({
            "market_id": order.market_id,
            "price": format!(
                "{}",
                (order.price * (10 as u128).pow(decimals.into()) as f64) as u128
            ),
            "quantity": format!(
                "{}",
                (order.quantity * (10 as u128).pow(decimals.into()) as f64) as u128
            ),
            "ttl": order.ttl,
            "market_order": order.market_order,
        })
        .to_string()
        .into_bytes(),
        gas: 100_000_000_000_000, // 100 TeraGas
        deposit: 0,
    })];
    // send 1 yocto
    // let actions: Vec<Action> = vec![Action::Transfer(TransferAction { deposit: 1 })];
    // println!("{:#?}", actions);
    let tx: Transaction = create_tx::run(client, signer, actions, "app_2.spin_swap.testnet")
        .await
        .expect("failed");
    // println!("{:#?}", tx);
    send_tx::run(client, signer, tx).await.expect("failed send");
    Ok(())
}
