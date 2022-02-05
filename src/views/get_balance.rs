use near_jsonrpc_client::auth::Unauthenticated;
use near_jsonrpc_client::{methods, JsonRpcClient};
use near_jsonrpc_primitives::types::query::QueryResponseKind;
use near_primitives::types::{BlockReference, Finality, FunctionArgs};
use near_primitives::views::QueryRequest;
use serde_json::{from_slice, json};

pub async fn run(
    client: &JsonRpcClient<Unauthenticated>,
    account_id: &str,
    token: &str,
) -> Result<String, &'static str> {
    let request = methods::query::RpcQueryRequest {
        block_reference: BlockReference::Finality(Finality::Final),
        request: QueryRequest::CallFunction {
            account_id: "app_2.spin_swap.testnet".parse().expect("fail parse"),
            method_name: "get_deposit".to_string(),
            args: FunctionArgs::from(
                json!({
                    "account_id": account_id,
                    "token": token,

                })
                .to_string()
                .into_bytes(),
            ),
        },
    };

    let response = client.call(request).await.expect("fail");

    match response.kind {
        QueryResponseKind::CallResult(res) => {
            let rest = from_slice::<String>(&res.result).expect("fail");
            println!("{:#?}", &rest);
            Ok(rest)
        }
        _ => Err("failed"),
    }
}
