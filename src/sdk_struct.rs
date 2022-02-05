use crate::sends::{
    post_ask, post_bid, post_cancel_all_orders, post_cancel_market_orders, post_cancel_order,
    post_deposit, post_withdraw,
};
use crate::utils::structs::{self, PlacedOrder};
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
impl SpinSDK {
    pub async fn send_deposit(
        &self,
        amount: f64,
        contract_id: &str,
        decimals: u8,
    ) -> Result<(), ()> {
        post_deposit::run(&self.client, &self.signer, amount, contract_id, decimals).await
    }
    pub async fn send_withdrawal(
        &self,
        amount: f64,
        contract_id: &str,
        decimals: u8,
    ) -> Result<(), ()> {
        post_withdraw::run(&self.client, &self.signer, amount, contract_id, decimals).await
    }
    pub async fn send_bid(&self, order: PlacedOrder, decimals: u8) -> Result<(), ()> {
        post_bid::run(&self.client, &self.signer, order, decimals).await
    }

    pub async fn send_ask(&self, order: PlacedOrder, decimals: u8) -> Result<(), ()> {
        post_ask::run(&self.client, &self.signer, order, decimals).await
    }

    pub async fn send_cancel_order(&self, market_id: u8, order_id: u64) -> Result<(), ()> {
        post_cancel_order::run(&self.client, &self.signer, market_id, order_id).await
    }

    pub async fn send_cancel_market_orders(&self, market_id: u8) -> Result<(), ()> {
        post_cancel_order::run(&self.client, &self.signer, market_id).await
    }

    pub async fn send_cancel_all_orders(&self) -> Result<(), ()> {
        post_cancel_order::run(&self.client, &self.signer).await
    }
}
