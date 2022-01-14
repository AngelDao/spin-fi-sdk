use near_jsonrpc_client::auth::Unauthenticated;
use near_jsonrpc_client::{methods, JsonRpcClient};
use near_jsonrpc_primitives::types::query::QueryResponseKind;
use near_primitives::types::{BlockReference, Finality, FunctionArgs};
use near_primitives::views::QueryRequest;
use serde::Deserialize;
use serde_json::{from_slice, json};

#[derive(Debug, Deserialize)]
pub struct Order {
    pub price: f64,
    pub quantity: f64,
}

pub type OrderList = Vec<Order>;

#[derive(Debug, Deserialize)]
pub struct SingleMarket {
    pub ask_orders: OrderList,
    pub bid_orders: OrderList,
}

pub async fn run(
    client: &JsonRpcClient<Unauthenticated>,
    id: u8,
) -> Result<SingleMarket, &'static str> {
    let request = methods::query::RpcQueryRequest {
        block_reference: BlockReference::Finality(Finality::Final),
        request: QueryRequest::CallFunction {
            account_id: "app.spin_swap.testnet".parse().expect("fail parse"),
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
            let rest = from_slice::<SingleMarket>(&res.result).expect("fail");
            println!("{:#?}", &rest);
            Ok(rest)
        }
        _ => Err("failed"),
    }
}
