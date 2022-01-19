use near_jsonrpc_client::auth::Unauthenticated;
use near_jsonrpc_client::{methods, JsonRpcClient};
use near_jsonrpc_primitives::types::query::QueryResponseKind;
use near_primitives::types::{BlockReference, Finality, FunctionArgs};
use near_primitives::views::QueryRequest;
use serde::Deserialize;
use serde_json::{from_slice, json};

#[derive(Debug, Deserialize)]
pub struct Order {
    pub id: u128,
    pub acc: String,
    pub price: u128,
    pub quantity: u128,
    pub left: u128,
    pub updated_at: u128,
    pub crate_at: u128,
    pub expiration_time: u128,
    pub o_type: String,
}

pub type AllOrders = Vec<Order>;

pub async fn run(
    client: &JsonRpcClient<Unauthenticated>,
    account_id: &str,
    market_id: u32,
) -> Result<AllOrders, &'static str> {
    let request = methods::query::RpcQueryRequest {
        block_reference: BlockReference::Finality(Finality::Final),
        request: QueryRequest::CallFunction {
            account_id: "app.spin_swap.testnet".parse().expect("fail parse"),
            method_name: "get_orders".to_string(),
            args: FunctionArgs::from(
                json!({
                    "market_id": market_id,
                    "account_id": account_id,
                })
                .to_string()
                .into_bytes(),
            ),
        },
    };

    let response = client.call(request).await.expect("failed call");

    match response.kind {
        QueryResponseKind::CallResult(res) => {
            let rest = from_slice::<AllOrders>(&res.result).expect("fail");
            println!("{:#?}", &rest);
            Ok(rest)
        }
        _ => {
            println!("failed");
            Err("failed")
        }
    }
}
