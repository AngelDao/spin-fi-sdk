use std::io::{self, Write};

use near_account_id::AccountId;
use near_crypto::{InMemorySigner, KeyType, SecretKey};
use near_jsonrpc_client::{methods, JsonRpcClient};
use near_jsonrpc_primitives::types::query::QueryResponseKind;
use near_jsonrpc_primitives::types::transactions::TransactionInfo;
use near_primitives::transaction::{Action, FunctionCallAction, Transaction};
use near_primitives::types::BlockReference;

use serde_json::json;
use tokio::time;

pub fn input(query: &str) -> io::Result<String> {
    print!("{}", query);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_owned())
}

pub fn run(id: &str, seed_phrase: &str) -> Result<InMemorySigner, ()> {
    // let signer_account_id: &str = id.parse().expect("failed");
    // let signer_secret_key: &str = seed_phrase.parse().expect("failed");
    let account_id: AccountId = id.parse().unwrap();
    let secret_key: SecretKey = SecretKey::from_seed(KeyType::SECP256K1, seed_phrase);
    // match secret_key {
    //     SecretKey::SECP256K1(res) => {
    //         println!("{:#?}", res[0])
    //     }
    //     _ => println!("failed"),
    // }
    // println!("{:#?}", pkey.parse::<SecretKey>().expect("fail"));
    let signer: InMemorySigner = InMemorySigner::from_secret_key(account_id, secret_key);
    // println!("{:#?}", signer.account_id);
    Ok(signer)
    // Ok(signer)
    // let access_key_query_response = client
    //     .call(methods::query::RpcQueryRequest {
    //         block_reference: BlockReference::latest(),
    //         request: near_primitives::views::QueryRequest::ViewAccessKey {
    //             account_id: signer.account_id.clone(),
    //             public_key: signer.public_key.clone(),
    //         },
    //     })
    //     .await?;

    // let current_nonce = match access_key_query_response.kind {
    //     QueryResponseKind::AccessKey(access_key) => access_key.nonce,
    //     _ => Err("failed to extract current nonce")?,
    // };

    // let other_account = utils::input("Enter the account to be rated: ")?;
    // let rating = utils::input("Enter a rating: ")?.parse::<f32>()?;

    // let transaction = Transaction {
    //     signer_id: signer.account_id.clone(),
    //     public_key: signer.public_key.clone(),
    //     nonce: current_nonce + 1,
    //     receiver_id: "nosedive.testnet".parse()?,
    //     block_hash: access_key_query_response.block_hash,
    //     actions: vec![Action::FunctionCall(FunctionCallAction {
    //         method_name: "rate".to_string(),
    //         args: json!({
    //             "account_id": other_account,
    //             "rating": rating,
    //         })
    //         .to_string()
    //         .into_bytes(),
    //         gas: 100_000_000_000_000, // 100 TeraGas
    //         deposit: 0,
    //     })],
    // };

    // let request = methods::broadcast_tx_async::RpcBroadcastTxAsyncRequest {
    //     signed_transaction: transaction.sign(&signer),
    // };

    // let sent_at = time::Instant::now();
    // let tx_hash = client.call(request).await?;

    // loop {
    //     let response = client
    //         .call(methods::tx::RpcTransactionStatusRequest {
    //             transaction_info: TransactionInfo::TransactionId {
    //                 hash: tx_hash,
    //                 account_id: signer.account_id.clone(),
    //             },
    //         })
    //         .await;
    //     let received_at = time::Instant::now();
    //     let delta = (received_at - sent_at).as_secs();

    //     if delta > 60 {
    //         Err("time limit exceeded for the transaction to be recognized")?;
    //     }

    //     match response {
    //         Err(err) => match err.handler_error()? {
    //             methods::tx::RpcTransactionError::UnknownTransaction { .. } => {
    //                 time::sleep(time::Duration::from_secs(2)).await;
    //                 continue;
    //             }
    //             err => Err(err)?,
    //         },
    //         Ok(response) => {
    //             println!("response gotten after: {}s", delta);
    //             println!("response: {:#?}", response);
    //             break;
    //         }
    //     }
    // }
}
