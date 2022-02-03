use near_jsonrpc_client::auth::Unauthenticated;
use near_jsonrpc_client::{methods, JsonRpcClient};
use near_jsonrpc_primitives::types::query::QueryResponseKind;
use near_primitives::types::{BlockReference, Finality, FunctionArgs};
use near_primitives::views::QueryRequest;
use serde::Deserialize;
use serde_json::{from_slice, json};

#[path = "../utils/structs.rs"]
pub mod structs;

pub async fn run(
    client: &JsonRpcClient<Unauthenticated>,
    id: u8,
) -> Result<structs::SingleMarket, &'static str> {
    let request = methods::query::RpcQueryRequest {
        block_reference: BlockReference::Finality(Finality::Final),
        request: QueryRequest::CallFunction {
            account_id: "app_2.spin_swap.testnet".parse().expect("fail parse"),
            method_name: "view_market".to_string(),
            args: FunctionArgs::from(
                json!({
                    "market_id": id,
                })
                .to_string()
                .into_bytes(),
            ),
        },
    };

    let response = client.call(request).await.expect("fail");

    match response.kind {
        QueryResponseKind::CallResult(res) => {
            let rest = from_slice::<structs::SingleMarket>(&res.result).expect("fail");
            println!("{:#?}", &rest);
            Ok(rest)
        }
        _ => Err("failed"),
    }
}