extern crate borsh;
extern crate near_account_id;
extern crate near_jsonrpc_client;
extern crate near_jsonrpc_primitives;
extern crate near_primitives;
extern crate serde;
extern crate serde_json;

mod connect_client;
mod connect_wallet;
mod get_balance;
mod get_balances;
mod get_local_seedphrase;
mod get_market;
mod get_markets;
mod post_ask;
mod post_bid;
mod post_deposit;

use near_crypto::InMemorySigner;

#[tokio::main]
async fn main() -> Result<(), ()> {
    let key: String = get_local_seedphrase::run().expect("failed");
    let signer: InMemorySigner = connect_wallet::run("danielw.testnet", &key).unwrap();
    let client = connect_client::run("https://rpc.testnet.near.org");
    post_bid::run(&client, &signer).await.expect("failed");
    // post_ask::run(&client, &signer).await.expect("failed");
    // post_deposit::run(&client, &signer).await.expect("failed");
    // get balance by accountid and token contract id
    // get_balance::run(&client, "danielw.testnet", "near.near")
    //     .await
    //     .unwrap();
    // get all balances
    get_balances::run(&client, "danielw.testnet").await.unwrap();
    // get all markets
    let markets: get_markets::AllMarkets = get_markets::run(&client).await.expect("failed");
    println!("{:#?}", markets);
    // get market by id
    // let market: get_market::SingleMarket = get_market::run(&client, markets[0].id)
    //     .await
    //     .expect("failed");
    Ok(())
}
