mod config;
mod sdk_struct;
mod sends;
mod utils;
mod views;

use crate::config::{config_client, get_local_seedphrase};
use crate::utils::structs::PlacedOrder;
use near_crypto::InMemorySigner;

#[tokio::main]
async fn main() -> Result<(), ()> {
    let key: String = get_local_seedphrase::run().expect("failed");
    let spin_sdk: sdk_struct::SpinSDK =
        config_client::initialize("https://rpc.testnet.near.org", "danielw.testnet", &key);
    spin_sdk.view_all_balances("danielw.testnet").await;
    spin_sdk.view_balance("danielw.testnet", "near.near").await;
    spin_sdk.view_all_markets().await;
    spin_sdk.view_market(1).await;
    // spin_sdk.view_all_orders("danielw.testnet", 1).await;
    // spin_sdk.view_order(1, 16474).await;
    // spin_sdk.send_deposit(1.0, "near.near", 24).await;
    // spin_sdk.send_deposit(0.70, "spfi_usdc.testnet", 24).await;
    // spin_sdk.send_withdrawal(1.0, "near.near", 24).await;
    // spin_sdk
    //     .send_withdrawal(10.0, "spfi_usdc.testnet", 24)
    //     .await;
    // let po = PlacedOrder {
    //     market_id: 1,
    //     price: 1.0,
    //     quantity: 1.0,
    //     // 1 day until expiry
    //     ttl: 60 * 60 * 24,
    //     market_order: false,
    // };
    // spin_sdk.send_bid(po, 24).await;
    // let po2 = PlacedOrder {
    //     market_id: 1,
    //     price: 9.0,
    //     quantity: 0.5,
    //     // 1 day until expiry
    //     ttl: 60 * 60 * 24,
    //     market_order: false,
    // };
    // spin_sdk.send_ask(po2, 24).await;
    Ok(())
}
