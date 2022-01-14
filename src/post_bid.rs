use std::io::{self, Write};

use near_account_id::AccountId;
use near_crypto::{InMemorySigner, KeyType, SecretKey};
use near_jsonrpc_client::auth::Unauthenticated;
use near_jsonrpc_client::{methods, JsonRpcClient};
use near_jsonrpc_primitives::types::query::QueryResponseKind;
use near_jsonrpc_primitives::types::transactions::TransactionInfo;
use near_primitives::transaction::{Action, FunctionCallAction, Transaction};
use near_primitives::types::BlockReference;

use serde_json::json;
use tokio::time;

pub async fn run(
    client: &JsonRpcClient<Unauthenticated>,
    signer: &InMemorySigner,
) -> Result<(), ()> {
    // what is this?
    let access_key_query_response = client
        .call(methods::query::RpcQueryRequest {
            block_reference: BlockReference::latest(),
            request: near_primitives::views::QueryRequest::ViewAccessKey {
                account_id: signer.account_id.clone(),
                public_key: signer.public_key.clone(),
            },
        })
        .await
        .expect("failed");

    let current_nonce = match access_key_query_response.kind {
        QueryResponseKind::AccessKey(access_key) => access_key.nonce,
        _ => Err("failed to extract current nonce")?,
    };

    let transaction = Transaction {
        signer_id: signer.account_id.clone(),
        public_key: signer.public_key.clone(),
        nonce: current_nonce + 1,
        receiver_id: "app.spin_swap.testnet".parse()?,
        block_hash: access_key_query_response.block_hash,
        // can I send multiple of these?
        actions: vec![Action::FunctionCall(FunctionCallAction {
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
        })],
    };

    let request = methods::broadcast_tx_async::RpcBroadcastTxAsyncRequest {
        signed_transaction: transaction.sign(&signer),
    };

    let sent_at = time::Instant::now();
    let tx_hash = client.call(request).await?;

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
