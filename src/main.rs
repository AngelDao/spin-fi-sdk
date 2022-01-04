extern crate near_jsonrpc_client;
extern crate near_jsonrpc_primitives;
extern crate near_primitives;
extern crate serde;
extern crate serde_json;

use near_jsonrpc_client::{methods, JsonRpcClient};
use near_jsonrpc_primitives::types::query::QueryResponseKind;
use near_jsonrpc_primitives::types::transactions::TransactionInfo;
use near_primitives::types::{BlockReference, Finality, FunctionArgs};
use near_primitives::views::{CallResult, QueryRequest};

use std::fmt::Debug;

use serde::de::Deserialize;
use serde_json::{from_slice, json};

use std::any::type_name;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    println!("Hello, world!");
    // mainnet
    // let mainnet_client = JsonRpcClient::connect("https://archival-rpc.mainnet.near.org");
    // testnet
    let testnet_client = JsonRpcClient::connect("https://archival-rpc.testnet.near.org");

    // let tx_status_request = methods::tx::RpcTransactionStatusRequest {
    //     transaction_info: TransactionInfo::TransactionId {
    //         hash: "9FtHUFBQsZ2MG77K3x3MJ9wjX3UT8zE1TczCrhZEcG8U"
    //             .parse()
    //             .expect("fail parse"),
    //         account_id: "miraclx.near".parse().expect("fail parse"),
    //     },
    // };

    // call a method on the server via the connected client
    // let tx_status = mainnet_client.call(tx_status_request).await;

    // println!("{:?}", tx_status);

    // let account_id = utils::input("app.spin_swap.testnet")?;

    let request = methods::query::RpcQueryRequest {
        block_reference: BlockReference::Finality(Finality::Final),
        request: QueryRequest::CallFunction {
            account_id: "app.spin_swap.testnet".parse().expect("fail parse"),
            method_name: "markets".to_string(),
            args: FunctionArgs::from("".to_string().into_bytes()),
        },
    };

    let response = testnet_client.call(request).await.expect("fail");
    // match response {
    //     Ok(res) => println!("success\n {:?}", res.kind.deserialize()),
    //     Err(e) => eprintln!("error :(\n {}", e),
    // }
    // println!("{:?}", type_of(response));
    if let QueryResponseKind::CallResult(result) = response.kind {
        let dres = result.result;
        println!("{:#?}", dres);
    }

    Ok(())
}
