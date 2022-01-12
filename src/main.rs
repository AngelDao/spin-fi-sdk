extern crate borsh;
extern crate near_jsonrpc_client;
extern crate near_jsonrpc_primitives;
extern crate near_primitives;
extern crate serde;
extern crate serde_json;

mod connect_client;
mod get_market;
mod get_markets;

use std::fs::File;
use std::io::prelude::*;

#[tokio::main]
async fn main() -> Result<(), ()> {
    let mut file = File::open("privkey.txt").expect("cant open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Oops!, cannot read file...");
    println!("file contents:\n\n{}", contents);
    // let client = connect_client::run("https://archival-rpc.testnet.near.org");
    // // get markets
    // let markets: get_markets::AllMarkets = get_markets::run(&client).await.expect("failed");
    // // println!("{:#?}", markets);
    // let market: get_market::SingleMarket = get_market::run(&client, markets[0].id)
    //     .await
    //     .expect("failed");
    Ok(())
}
