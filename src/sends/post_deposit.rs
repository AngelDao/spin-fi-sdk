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
    let mut actions: Vec<Action>;
    let mut contract_id: &str;

    if token == "near.near" {
        actions = vec![Action::FunctionCall(FunctionCallAction {
            method_name: "deposit_near".to_string(),
            args: json!({}).to_string().into_bytes(),
            gas: 100_000_000_000_000, // 100 TeraGas
            deposit: (amount * (10 as u128).pow(decimals.into()) as f64) as u128,
        })];
        contract_id = "app_2.spin_swap.testnet";
    } else {
        actions = vec![Action::FunctionCall(FunctionCallAction {
            method_name: "ft_transfer_call".to_string(),
            args: json!({
              "receiver_id": "app_2.spin_swap.testnet",
              "amount": format!("{}",(amount * (10 as u128).pow(decimals.into()) as f64) as u128),
              "msg": ""
            })
            .to_string()
            .into_bytes(),
            gas: 100_000_000_000_000, // 100 TeraGas
            deposit: 1,
        })];
        contract_id = token;
    }

    // send 1 yocto
    // let actions: Vec<Action> = vec![Action::Transfer(TransferAction { deposit: 1 })];
    // println!("{:#?}", actions);
    println!("{:#?}", contract_id);
    let tx: Transaction = create_tx::run(client, signer, actions, contract_id)
        .await
        .expect("failed");
    println!("{:#?}", tx);
    send_tx::run(client, signer, tx).await.expect("failed send");
    Ok(())
}
