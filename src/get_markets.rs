use near_jsonrpc_client::auth::Unauthenticated;
use near_jsonrpc_client::{methods, JsonRpcClient};
use near_jsonrpc_primitives::types::query::QueryResponseKind;
use near_primitives::types::{BlockReference, Finality, FunctionArgs};
use near_primitives::views::QueryRequest;
use serde::Deserialize;
use serde_json::from_slice;

#[derive(Debug, Deserialize)]
pub struct Base {
    pub ticker: String,
    pub decimal: u8,
}

#[derive(Debug, Deserialize)]
pub struct Quote {
    pub ticker: String,
    pub decimal: u8,
}

#[derive(Debug, Deserialize)]
pub struct Market {
    pub id: u8,
    pub base: Base,
    pub quote: Quote,
}

pub type AllMarkets = Vec<Market>;

pub async fn run(client: &JsonRpcClient<Unauthenticated>) -> Result<AllMarkets, &'static str> {
    let request = methods::query::RpcQueryRequest {
        block_reference: BlockReference::Finality(Finality::Final),
        request: QueryRequest::CallFunction {
            account_id: "app.spin_swap.testnet".parse().expect("fail parse"),
            method_name: "markets".to_string(),
            args: FunctionArgs::from("".to_string().into_bytes()),
        },
    };

    let response = client.call(request).await.expect("fail");

    match response.kind {
        QueryResponseKind::CallResult(res) => {
            let rest = from_slice::<AllMarkets>(&res.result).expect("fail");
            let dres = &rest.len();
            println!("{:#?}", &rest[1]);
            Ok(rest)
        }
        _ => Err("failed"),
    }
}
