extern crate borsh;
extern crate near_account_id;
extern crate near_jsonrpc_client;
extern crate near_jsonrpc_primitives;
extern crate near_primitives;
extern crate serde;
extern crate serde_json;

mod sdk_struct;

use near_crypto::InMemorySigner;

#[tokio::main]
async fn main() -> Result<(), ()> {
    let key: String = get_local_seedphrase::run().expect("failed");
    // let signer: InMemorySigner = connect_wallet::run("danielw.testnet", &key).unwrap();
    // let client = connect_client::run("https://rpc.testnet.near.org");
    let spin_sdk: config_client::sdk_struct::SpinSDK =
        config_client::initialize("https://rpc.testnet.near.org", "danielw.testnet", &key);
    spin_sdk.view_all_balances("danielw.testnet").await;
    spin_sdk.view_balance("danielw.testnet", "near.near").await;
    spin_sdk.view_all_markets().await;
    spin_sdk.view_market(1).await;
    spin_sdk.view_all_orders("danielw.testnet", 1).await;
    spin_sdk.view_order(1, 1).await;
    // post_bid::run(&client, &signer).await.expect("failed");
    // post_ask::run(&client, &signer).await.expect("failed");
    // post_deposit::run(&client, &signer).await.expect("failed");
    // post_withdraw::run(&client, &signer).await.expect("failed");
    // get orders by market id
    // get_orders::run(&client, "danielw.testnet", 1)
    //     .await
    //     .unwrap();
    // get order by market id and order id
    // get_order::run(&client, 1, 1).await.unwrap();
    // get balance by accountid and token contract id
    // get_balance::run(&client, "danielw.testnet", "near.near")
    //     .await
    //     .unwrap();
    // get all balances
    // get_balances::run(&client, "danielw.testnet").await.unwrap();
    // get all markets
    // let markets: get_markets::AllMarkets = get_markets::run(&client).await.expect("failed");
    // println!("{:#?}", markets);
    // get market by id
    // let market: get_market::SingleMarket = get_market::run(&client, markets[0].id)
    //     .await
    //     .expect("failed");
    Ok(())
}
