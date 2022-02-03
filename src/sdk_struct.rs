use near_crypto::InMemorySigner;
use near_jsonrpc_client::auth::Unauthenticated;
use near_jsonrpc_client::{methods, JsonRpcClient};
use serde::Deserialize;

#[path = "./views/get_balance.rs"]
mod get_balance;
#[path = "./views/get_balances.rs"]
mod get_balances;
#[path = "./views/get_market.rs"]
mod get_market;
#[path = "./views/get_markets.rs"]
mod get_markets;
#[path = "./views/get_order.rs"]
mod get_order;
#[path = "./views/get_orders.rs"]
mod get_orders;

pub struct SpinSDK {
    pub client: JsonRpcClient<Unauthenticated>,
    pub signer: InMemorySigner,
}

impl SpinSDK {
    pub async fn view_all_balances(&self, account_id: &str) -> Result<(), &'static str> {
        get_balances::run(&self.client, account_id).await
    }
    pub async fn view_balance(
        &self,
        account_id: &str,
        token: &str,
    ) -> Result<String, &'static str> {
        get_balance::run(&self.client, account_id, token).await
    }
    pub async fn view_all_markets(&self) -> Result<get_markets::AllMarkets, &'static str> {
        get_markets::run(&self.client).await
    }
    pub async fn view_market(
        &self,
        market_id: u8,
    ) -> Result<get_market::structs::SingleMarket, &'static str> {
        get_market::run(&self.client, market_id).await
    }
    pub async fn view_all_orders(
        &self,
        account_id: &str,
        market_id: u32,
    ) -> Result<get_order::structs::SingleOrder, &'static str> {
        get_orders::run(&self.client, account_id, market_id).await
    }
    pub async fn view_order(
        &self,
        market_id: u32,
        order_id: u32,
    ) -> Result<get_order::structs::SingleOrder, &'static str> {
        get_order::run(&self.client, market_id, order_id).await
    }
}
