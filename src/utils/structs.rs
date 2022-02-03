use serde::Deserialize;

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
    pub crate_at: u128,
    pub expiration_time: u128,
    pub o_type: String,
}

pub type AllOrders = Vec<SingleOrder>;

pub type AllMarkets = Vec<Market>;
