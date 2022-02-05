use crate::utils::{create_tx, send_tx};
use near_crypto::{InMemorySigner, KeyType, SecretKey};
use near_jsonrpc_client::auth::Unauthenticated;
use near_jsonrpc_client::JsonRpcClient;
use near_primitives::transaction::Transaction;
use near_primitives::transaction::{Action, FunctionCallAction, TransferAction};
use serde_json::json;

pub async fn run(
    client: &JsonRpcClient<Unauthenticated>,
    signer: &InMemorySigner,
    amount: f64,
    token: &str,
    decimals: u8,
) -> Result<(), ()> {
    let mut deposit_val: u128;

    if token == "near.near" {
        deposit_val = 0;
    } else {
        deposit_val = 1;
    };
    let actions: Vec<Action> = vec![Action::FunctionCall(FunctionCallAction {
        method_name: "withdraw".to_string(),
        args: json!({
            "token": token,
            "amount": format!("{}",(amount * (10 as u128).pow(decimals.into()) as f64) as u128),
        })
        .to_string()
        .into_bytes(),
        gas: 100_000_000_000_000, // 100 TeraGas
        deposit: deposit_val,
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
