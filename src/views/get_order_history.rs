#[path = "../utils/structs.rs"]
use crate::utils::structs;
use near_jsonrpc_client::auth::Unauthenticated;
use near_jsonrpc_client::{methods, JsonRpcClient};
use near_jsonrpc_primitives::types::query::QueryResponseKind;
use near_primitives::types::{BlockReference, Finality, FunctionArgs};
use near_primitives::views::QueryRequest;
use serde::Deserialize;
use serde_json::{from_slice, json};
// pub mod structs;

pub async fn run(
    client: &JsonRpcClient<Unauthenticated>,
    account_id: &str,
    market_id: u32,
) -> Result<structs::OrderHistory, &'static str> {
    let request = methods::query::RpcQueryRequest {
        block_reference: BlockReference::Finality(Finality::Final),
        request: QueryRequest::CallFunction {
            account_id: "app_2.spin_swap.testnet"
                .parse()
                .expect("failed accountid parse"),
            method_name: "get_order_history".to_string(),
            args: FunctionArgs::from(
                json!(  {
                    "market_id": market_id,
                    "account_id": account_id,
                    "limit": 100,
                    "offset": 1
                })
                .to_string()
                .into_bytes(),
            ),
        },
    };

    let response = client.call(request).await.unwrap();

    match response.kind {
        QueryResponseKind::CallResult(res) => {
            let rest = from_slice::<structs::OrderHistory>(&res.result).expect("fail deserialize");
            println!("{:#?}", &rest);
            Ok(rest)
        }
        _ => {
            println!("failed");
            Err("failed")
        }
    }
}