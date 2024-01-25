use chrono::Utc;
use colored::*;
use dialoguer::{theme::ColorfulTheme, MultiSelect};
use reqwest::Client;
use tokio;

// include my module
mod order;
use order::{CoinbaseOrder, GeminiOrder, KrakenResult};

mod order_book;
use crate::order_book::*;

mod utils;
use crate::utils::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // use std::time::SystemTime;
    let client = Client::new();
    // let term = Term::stdout();

    let mut selections;

    // let selections = MultiSelect::with_theme(&ColorfulTheme::default())
    //     .with_prompt("Please select which market api that you want to fetch data from")
    //     .items(&["Coinbase", "Gemini", "Kraken"])
    //     .interact()?;

    loop {
        selections = MultiSelect::with_theme(&ColorfulTheme::default())
            .with_prompt("Please select which market api that you want to fetch data from")
            .items(&["Coinbase", "Gemini", "Kraken"])
            .interact()?;

        if selections.is_empty() {
            println!("You have to select at least one!");
        } else {
            break;
        }
    }

    let mut coinbase_book: Option<OrderBook> = None;
    let mut gemini_book: Option<OrderBook> = None;
    let mut kraken_book: Option<OrderBook> = None;

    for market in selections {
        match market {
            0 => {
                println!("Fetching data from Coinbase");
                let coinbase_resp: CoinbaseOrder = client
                    .get("https://api.pro.coinbase.com/products/BTC-USD/book?level=2")
                    .send()
                    .await?
                    .json()
                    .await?;
                coinbase_book = Some(coinbase_resp.into());
            }
            1 => {
                println!("Fetching data from Gemini");
                let gemini_resp: GeminiOrder = client
                    .get("https://api.gemini.com/v1/book/BTCUSD")
                    .send()
                    .await?
                    .json()
                    .await?;
                gemini_book = Some(gemini_resp.into());
            }
            2 => {
                println!("Fetching data from Kraken");
                let kraken_resp: KrakenResult = client
                    .get("https://api.kraken.com/0/public/Depth?pair=XBTUSD")
                    .send()
                    .await?
                    .json()
                    .await?;
                kraken_book = Some(kraken_resp.into());
            }
            _ => {
                println!("You have to select at least one!");
            }
        }
    }

    // Merge order books and order by price descending for bids
    let mut merged_bids = {
        let mut merged_bids = Vec::new();
        if let Some(book) = &mut coinbase_book {
            merged_bids.append(&mut book.bids);
        }
        if let Some(book) = &mut gemini_book {
            merged_bids.append(&mut book.bids);
        }
        if let Some(book) = &mut kraken_book {
            merged_bids.append(&mut book.bids);
        }
        merged_bids
    };

    let now = Utc::now();
    println!(
        "{}",
        now.format("%Y-%m-%dT%H:%M:%S%.6fZ")
            .to_string()
            .bright_green()
            .bold()
    );

    merged_bids.sort_by(|a, b| b.price.partial_cmp(&a.price).unwrap());

    // Calculate selling price
    let price_to_sell = calculate_price(&mut merged_bids, 10.0);
    println!(
        "Price to sell 10 BTC across selected markets: {}",
        price_to_sell
    );

    // Merge order books and order by price ascending for asks
    let mut merged_asks = {
        let mut merged_asks = Vec::new();
        if let Some(book) = &mut coinbase_book {
            merged_asks.append(&mut book.asks);
        }
        if let Some(book) = &mut gemini_book {
            merged_asks.append(&mut book.asks);
        }
        if let Some(book) = &mut kraken_book {
            merged_asks.append(&mut book.asks);
        }
        merged_asks
    };

    merged_asks.sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap());

    // Calculate buying price
    let price_to_buy = calculate_price(&mut merged_asks, 10.0);
    println!(
        "Price to buy 10 BTC across selected markets: {}",
        price_to_buy
    );

    Ok(())
}
