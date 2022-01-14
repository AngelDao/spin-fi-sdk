extern crate borsh;
extern crate near_account_id;
extern crate near_jsonrpc_client;
extern crate near_jsonrpc_primitives;
extern crate near_primitives;
extern crate serde;
extern crate serde_json;

mod connect_client;
mod connect_wallet;
mod get_local_seedphrase;
mod get_market;
mod get_markets;

use near_crypto::InMemorySigner;

#[tokio::main]
async fn main() -> Result<(), ()> {
    let key: String = get_local_seedphrase::run().expect("failed");
    let signer: InMemorySigner = connect_wallet::run("danielw.testnet", &key).expect("failed");
    println!("{:#?}", signer.account_id);

    // let client = connect_client::run("https://archival-rpc.testnet.near.org");
    // // get markets
    // let markets: get_markets::AllMarkets = get_markets::run(&client).await.expect("failed");
    // // println!("{:#?}", markets);
    // let market: get_market::SingleMarket = get_market::run(&client, markets[0].id)
    //     .await
    //     .expect("failed");
    Ok(())
}
