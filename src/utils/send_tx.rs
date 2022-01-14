use near_crypto::{InMemorySigner, KeyType, SecretKey};
use near_jsonrpc_client::auth::Unauthenticated;
use near_jsonrpc_client::{methods, JsonRpcClient};
use near_jsonrpc_primitives::types::transactions::TransactionInfo;
use near_primitives::transaction::Transaction;
use tokio::time;

pub async fn run(
    client: &JsonRpcClient<Unauthenticated>,
    signer: InMemorySigner,
    transaction: Transaction,
) -> Result<(), &str> {
    let request = methods::broadcast_tx_async::RpcBroadcastTxAsyncRequest {
        signed_transaction: transaction.sign(&signer),
    };

    let sent_at = time::Instant::now();
    let tx_hash = client.call(request).await?;

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

        if delta > 60 {
            Err("time limit exceeded for the transaction to be recognized");
        }

        match response {
            Err(err) => match err.handler_error()? {
                methods::tx::RpcTransactionError::UnknownTransaction { .. } => {
                    time::sleep(time::Duration::from_secs(2)).await;
                    continue;
                }
                err => Err("failed"),
            },
            Ok(response) => {
                println!("response gotten after: {}s", delta);
                println!("response: {:#?}", response);
                break;
            }
        }
    }
    Ok(())
}
