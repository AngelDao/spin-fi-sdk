use near_crypto::{InMemorySigner, KeyType, SecretKey};
use near_jsonrpc_client::auth::Unauthenticated;
use near_jsonrpc_client::{methods, JsonRpcClient};
use near_jsonrpc_primitives::types::query::QueryResponseKind;
use near_primitives::hash::CryptoHash;
use near_primitives::transaction::{Action, Transaction};
use near_primitives::types::{BlockReference, Nonce};
use near_primitives::views::QueryRequest;

pub async fn run(
    client: &JsonRpcClient<Unauthenticated>,
    signer: &InMemorySigner,
    actions: Vec<Action>,
) -> Result<Transaction, &'static str> {
    let access_key_query_response = client
        .call(methods::query::RpcQueryRequest {
            block_reference: BlockReference::latest(),
            request: QueryRequest::ViewAccessKeyList {
                account_id: signer.account_id.clone(),
            },
        })
        .await
        .expect("failed request");

    let mut nonce: Nonce;
    let mut block_hash: CryptoHash;
    let mut tx: Transaction;
    if let QueryResponseKind::AccessKeyList(result) = access_key_query_response.kind {
        println!("{:#?}", result.keys[0].public_key.clone());
        let access_key_query_response_1 = client
            .call(methods::query::RpcQueryRequest {
                block_reference: BlockReference::latest(),
                request: QueryRequest::ViewAccessKey {
                    account_id: signer.account_id.clone(),
                    public_key: result.keys[0].public_key.clone(),
                },
            })
            .await
            .expect("failed request");
        block_hash = access_key_query_response_1.block_hash;
        if let QueryResponseKind::AccessKey(result_1) = access_key_query_response_1.kind {
            println!("{:#?}", result_1);
            nonce = result_1.nonce;
            println!("block hash{:#?}", block_hash);
            println!("nonce {:#?}", nonce);
            let tx = Transaction {
                signer_id: signer.account_id.clone(),
                public_key: signer.public_key.clone(),
                nonce: nonce + 1,
                block_hash,
                receiver_id: "app.spin_swap.testnet".parse().expect("failed type"),
                actions: actions,
            };
            Ok(tx)
        } else {
            Err("failed")
        }
    } else {
        Err("failed")
    }
}
