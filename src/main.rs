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
mod post_bid;

use near_crypto::InMemorySigner;

#[tokio::main]
async fn main() -> Result<(), ()> {
    let key: String = get_local_seedphrase::run().expect("failed");
    println!("{:#?}", &key);
    let signer: InMemorySigner = connect_wallet::run("danielw.testnet", &key).unwrap();
    println!(
        "pub {:#?}\npriv {:#?}\nacc {:#?}",
        signer.public_key.clone(),
        signer.secret_key.clone(),
        signer.account_id.clone()
    );
    let client = connect_client::run("https://archival-rpc.testnet.near.org");
    post_bid::run(&client, &signer).await.expect("failed");
    // // get markets
    // let markets: get_markets::AllMarkets = get_markets::run(&client).await.expect("failed");
    // println!("{:#?}", markets);
    // let market: get_market::SingleMarket = get_market::run(&client, markets[0].id)
    //     .await
    //     .expect("failed");
    Ok(())
}
