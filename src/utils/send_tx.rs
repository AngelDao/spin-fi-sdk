use near_crypto::{InMemorySigner, KeyType, SecretKey};
use near_jsonrpc_client::auth::Unauthenticated;
use near_jsonrpc_client::{methods, JsonRpcClient};
use near_jsonrpc_primitives::types::transactions::TransactionInfo;
use near_primitives::transaction::Transaction;
use tokio::time;
#[path = "../connect_client.rs"]
mod connect_client;

pub async fn run(
    client: &JsonRpcClient<Unauthenticated>,
    signer: &InMemorySigner,
    transaction: Transaction,
) -> Result<(), &'static str> {
    let signed_tx = transaction.sign(signer);
    let request = methods::broadcast_tx_async::RpcBroadcastTxAsyncRequest {
        signed_transaction: signed_tx,
    };

    // println!("request {:#?}", request);

    let sent_at = time::Instant::now();
    let tx_hash = client.call(request).await.unwrap();
    println!("{:#?}", tx_hash);
    loop {
        let response = client
            .call(methods::tx::RpcTransactionStatusRequest {
                transaction_info: TransactionInfo::TransactionId {
                    hash: tx_hash,
                    account_id: signer.account_id.clone(),
                },
            })
            .await;
        let received_at = time::Instant::now();
        let delta = (received_at - sent_at).as_secs();
        println!("{:#?}", delta);
        if delta > 60 {
            println!("time limit exceeded for the transaction to be recognized");
            break;
        }

        match response {
            Err(err) => {
                println!("{:#?}", err);
                match err.handler_error().expect("failed") {
                    methods::tx::RpcTransactionError::UnknownTransaction { .. } => {
                        time::sleep(time::Duration::from_secs(2)).await;
                        continue;
                    }
                    err => {
                        println!("failed {:#?}", err);
                        break;
                    }
                }
            }
            Ok(response) => {
                println!("response gotten after: {}s", delta);
                println!("response: {:#?}", response);
                break;
            }
        }
    }
    Ok(())
}
