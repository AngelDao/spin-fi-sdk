use crate::utils::structs;
use near_jsonrpc_client::auth::Unauthenticated;
use near_jsonrpc_client::{methods, JsonRpcClient};
use near_jsonrpc_primitives::types::query::QueryResponseKind;
use near_primitives::types::{BlockReference, Finality, FunctionArgs};
use near_primitives::views::QueryRequest;
use serde::Deserialize;
use serde_json::{from_slice, json};

pub async fn run(
    client: &JsonRpcClient<Unauthenticated>,
    market_id: u32,
    order_id: u32,
) -> Result<structs::SingleOrder, &'static str> {
    let request = methods::query::RpcQueryRequest {
        block_reference: BlockReference::Finality(Finality::Final),
        request: QueryRequest::CallFunction {
            account_id: "app_2.spin_swap.testnet".parse().expect("fail parse"),
            method_name: "get_order_by_id".to_string(),
            args: FunctionArgs::from(
                json!({
                    "market_id": market_id,
                    "order_id": order_id,
                })
                .to_string()
                .into_bytes(),
            ),
        },
    };

    let response = client.call(request).await.expect("failed call");

    match response.kind {
        QueryResponseKind::CallResult(res) => {
            let rest = from_slice::<structs::SingleOrder>(&res.result).expect("fail");
            println!("{:#?}", &rest);
            Ok(rest)
        }
        _ => {
            println!("failed");
            Err("failed")
        }
    }
}
