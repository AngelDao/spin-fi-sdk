use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct Base {
    pub ticker: String,
    pub decimal: u8,
}

#[derive(Debug, Deserialize)]
pub struct Quote {
    pub ticker: String,
    pub decimal: u8,
}

#[derive(Debug, Deserialize)]
pub struct Market {
    pub id: u8,
    pub base: Base,
    pub quote: Quote,
}

#[derive(Debug, Deserialize)]
pub struct Order {
    pub price: f64,
    pub quantity: f64,
}

pub type OrderList = Vec<Order>;

#[derive(Debug, Deserialize)]
pub struct SingleMarket {
    pub ask_orders: OrderList,
    pub bid_orders: OrderList,
}

#[derive(Debug, Deserialize)]
pub struct SingleOrder {
    pub id: u128,
    pub acc: String,
    pub price: u128,
    pub quantity: u128,
    pub left: u128,
    pub updated_at: u128,
    pub created_at: u128,
    pub expiration_time: u128,
    pub o_type: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PlacedOrder {
    pub market_id: u8,
    pub price: f64,
    pub quantity: f64,
    pub ttl: u64,
    pub market_order: bool,
}

#[derive(Debug, Deserialize)]
pub struct HistoricalOrder {
    id: u64,
    side: String,
    o_type: String,
    execution_price: u128,
    price: u128,
    quantity: u128,
    status: String,
    remaining: u128,
    created_at: u128,
    updated_at: u128,
}

pub type OrderHistory = Vec<HistoricalOrder>;

pub type AllOrders = Vec<SingleOrder>;

pub type AllMarkets = Vec<Market>;
