use crate::order_book::{Order, OrderBook};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct CoinbaseOrder {
    bids: Vec<(String, String, u64)>,
    asks: Vec<(String, String, u64)>,
    // other fields
}

#[derive(Debug, Deserialize)]
pub struct GeminiOrder {
    bids: Vec<GeminiBid>,
    asks: Vec<GeminiBid>,
    // other fields
}

#[derive(Debug, Deserialize)]
pub struct GeminiBid {
    price: String,
    amount: String,
    // other fields
}

#[derive(Debug, Deserialize)]
pub struct KrakenResult {
    result: HashMap<String, KrakenPair>,
}

#[derive(Debug, Deserialize)]
pub struct KrakenPair {
    bids: Vec<(String, String, u64)>,
    asks: Vec<(String, String, u64)>,
}

// To define the methods to convert the raw API data into the common format,
// we can implement the From trait for transforming the specific exchange order
// struct to generic Order.
impl From<CoinbaseOrder> for OrderBook {
    fn from(raw: CoinbaseOrder) -> Self {
        let bids = raw
            .bids
            .into_iter()
            .map(|entry| Order {
                price: entry.0.parse().unwrap(),
                quantity: entry.1.parse().unwrap(),
            })
            .collect();

        let asks = raw
            .asks
            .into_iter()
            .map(|entry| Order {
                price: entry.0.parse().unwrap(),
                quantity: entry.1.parse().unwrap(),
            })
            .collect();

        OrderBook { bids, asks }
    }
}

// This impl block provides a conversion from the GeminiOrder
// structure (i.e., the format in which data is received
// from the Gemini exchange API) to our internal OrderBook
// structure. This conversion is necessary to provide an
// exchange-agnostic internal representation of order data.
// This simplifies the following data processing steps,
// since we can use a unified approach to handle order
// book data, regardless of which exchange it originates from.
impl From<GeminiOrder> for OrderBook {
    fn from(raw: GeminiOrder) -> Self {
        let bids = raw
            .bids
            .into_iter()
            .map(|entry| Order {
                price: entry.price.parse().unwrap(),
                quantity: entry.amount.parse().unwrap(),
            })
            .collect();

        let asks = raw
            .asks
            .into_iter()
            .map(|entry| Order {
                price: entry.price.parse().unwrap(),
                quantity: entry.amount.parse().unwrap(),
            })
            .collect();

        OrderBook { bids, asks }
    }
}

// This impl block provides a conversion from the KrakenResult
// structure (i.e., the format in which data is received
// from the Kraken exchange API) to our internal OrderBook
// structure. This conversion is necessary to provide an
// exchange-agnostic internal representation of order data.
// This simplifies the following data processing steps,
// since we can use a unified approach to handle order
// book data, regardless of which exchange it originates from.
impl From<KrakenResult> for OrderBook {
    fn from(raw: KrakenResult) -> Self {
        let pair = raw.result.values().next().unwrap();

        let bids = pair
            .bids
            .iter()
            .map(|entry| Order {
                price: entry.0.parse().unwrap(),
                quantity: entry.1.parse().unwrap(),
            })
            .collect();

        let asks = pair
            .asks
            .iter()
            .map(|entry| Order {
                price: entry.0.parse().unwrap(),
                quantity: entry.1.parse().unwrap(),
            })
            .collect();

        OrderBook { bids, asks }
    }
}

// coinbase's response format and api are

// {
//     "bids": [
//         [
//             "39982.8",
//             "0.01884514",
//             1
//         ],
//         ...
//     ],
//     "asks": [
//         [
//             "39985.08",
//             "0.07946493",
//             1
//         ],
//         ...
//     ],
//     "sequence": 72011663386,
//     "auction_mode": false,
//     "auction": null,
//     "time": "2024-01-24T15:32:52.961744Z"
// }

// Gemini's response format and restapi are
// {
//     "bids": [
//         {
//             "price": "39950.0",
//             "amount": "0.00011063",
//             "timestamp": "1706112724"
//         },
//         {
//         ...
//     ],
//     "asks": [
//         {
//             "price": "39950.01",
//             "amount": "0.10070212",
//             "timestamp": "1706112724"
//         },
//         ...
//     ],
// }

// Kraken's response format and restapi are
// {
//     "error": [],
//     "result": {
//         "XXBTZUSD": {
//             "asks": [
//                 [
//                     "39971.30000",
//                     "2.606",
//                     1706112893
//                 ],
//                 ...
//             ]
//         },
//         "bids": [
//             [
//                 "39971.20000",
//                 "2.078",
//                 1706112894
//             ],
//             ...
//         ]
//     }
// }
