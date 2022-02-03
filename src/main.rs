mod config;
mod sdk_struct;
mod utils;
mod views;

use crate::config::{config_client, get_local_seedphrase};
use near_crypto::InMemorySigner;

#[tokio::main]
async fn main() -> Result<(), ()> {
    let key: String = get_local_seedphrase::run().expect("failed");
    // let signer: InMemorySigner = connect_wallet::run("danielw.testnet", &key).unwrap();
    // let client = connect_client::run("https://rpc.testnet.near.org");
    let spin_sdk: sdk_struct::SpinSDK =
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
    Ok(())
}
