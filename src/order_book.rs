#[derive(Debug, Clone, PartialEq)]
pub struct Order {
    pub exchange_name: String,
    pub price: f64,
    pub quantity: f64,
}

#[derive(Debug, Clone)]
pub struct OrderBook {
    pub bids: Vec<Order>,
    pub asks: Vec<Order>,
}
