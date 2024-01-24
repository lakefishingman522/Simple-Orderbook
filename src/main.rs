use reqwest::Client;
// use serde_json::Value;
use tokio;

// include my module
mod order;
use order::{CoinbaseOrder, GeminiOrder, KrakenResult};

mod order_book;
use crate::order_book::*;

mod utils;
use crate::utils::*; // Imports everything from `utils.rs`

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    // Fetch Coinbase data and normalize it
    let coinbase_resp: CoinbaseOrder = client
        .get("https://api.pro.coinbase.com/products/BTC-USD/book?level=2")
        .send()
        .await?
        .json()
        .await?;
    let mut coinbase_book: OrderBook = coinbase_resp.into();

    // Fetch Gemini data
    let gemini_resp: GeminiOrder = client
        .get("https://api.gemini.com/v1/book/BTCUSD")
        .send()
        .await?
        .json()
        .await?;
    let mut gemini_book: OrderBook = gemini_resp.into();

    let kraken_resp: KrakenResult = client
        .get("https://api.kraken.com/0/public/Depth?pair=XBTUSD")
        .send()
        .await?
        .json()
        .await?;
    let mut kraken_book: OrderBook = kraken_resp.into();

    // Merge order books and order by price descending for bids
    let mut merged_bids = [
        &mut coinbase_book.bids[..],
        &mut gemini_book.bids[..],
        &mut kraken_book.bids[..],
    ]
    .concat();
    merged_bids.sort_by(|a, b| b.price.partial_cmp(&a.price).unwrap());

    // Calculate selling price
    let price_to_sell = calculate_price(&mut merged_bids, 10.0);
    println!(
        "Price to sell 10 BTC across Coinbase, Gemini and Kraken: {}",
        price_to_sell
    );

    // Merge order books and order by price ascending for asks
    let mut merged_asks = [
        &mut coinbase_book.asks[..],
        &mut gemini_book.asks[..],
        &mut kraken_book.asks[..],
    ]
    .concat();
    merged_asks.sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap());

    // Calculate buying price
    let price_to_buy = calculate_price(&mut merged_asks, 10.0);
    println!(
        "Price to buy 10 BTC across Coinbase, Gemini and Kraken: {}",
        price_to_buy
    );

    Ok(())
}
