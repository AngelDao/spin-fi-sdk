use crate::utils::structs;
use crate::views::{get_balance, get_balances, get_market, get_markets, get_order, get_orders};
use near_crypto::InMemorySigner;
use near_jsonrpc_client::auth::Unauthenticated;
use near_jsonrpc_client::{methods, JsonRpcClient};
use serde::Deserialize;

pub struct SpinSDK {
    pub client: JsonRpcClient<Unauthenticated>,
    pub signer: InMemorySigner,
}

// views
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
    pub async fn view_all_markets(&self) -> Result<structs::AllMarkets, &'static str> {
        get_markets::run(&self.client).await
    }
    pub async fn view_market(&self, market_id: u8) -> Result<structs::SingleMarket, &'static str> {
        get_market::run(&self.client, market_id).await
    }
    pub async fn view_all_orders(
        &self,
        account_id: &str,
        market_id: u32,
    ) -> Result<structs::AllOrders, &'static str> {
        get_orders::run(&self.client, account_id, market_id).await
    }
    pub async fn view_order(
        &self,
        market_id: u32,
        order_id: u32,
    ) -> Result<structs::SingleOrder, &'static str> {
        get_order::run(&self.client, market_id, order_id).await
    }
}

// // send
// impl SpinSDK {
//     pub async fn
// }
