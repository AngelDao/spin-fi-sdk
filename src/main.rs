extern crate borsh;
extern crate near_jsonrpc_client;
extern crate near_jsonrpc_primitives;
extern crate near_primitives;
extern crate serde;
extern crate serde_json;

mod connect_client;
mod get_markets;

#[tokio::main]
async fn main() -> Result<(), ()> {
    let client = connect_client::run("https://archival-rpc.testnet.near.org");
    // get markets
    let markets: get_markets::AllMarkets = get_markets::run(client).await.expect("failed");
    // println!("{:#?}", markets);
    Ok(())
}
