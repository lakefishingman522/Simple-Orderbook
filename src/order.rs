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

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_calculate_merged_bids() {
        // Example data for each exchange. Please use your actual data here.
        let coinbase_order = CoinbaseOrder {
            bids: vec![
                ("40000".to_string(), "0.1".to_string(), 1),
                ("39900".to_string(), "0.2".to_string(), 2),
            ],
            asks: vec![
                ("40001".to_string(), "0.1".to_string(), 1),
                ("39901".to_string(), "0.2".to_string(), 2),
            ],
        };

        let gemini_order = GeminiOrder {
            bids: vec![
                GeminiBid {
                    price: "40050".into(),
                    amount: "0.2".into(),
                },
                GeminiBid {
                    price: "39950".into(),
                    amount: "0.2".into(),
                },
            ],
            asks: vec![
                GeminiBid {
                    price: "40051".into(),
                    amount: "1.2".into(),
                },
                GeminiBid {
                    price: "39951".into(),
                    amount: "1.2".into(),
                },
            ],
        };

        let mut kraken_data = HashMap::new();
        kraken_data.insert(
            "XXBTZUSD".into(),
            KrakenPair {
                bids: vec![
                    ("40000".to_string(), "0.1".to_string(), 1),
                    ("39900".to_string(), "0.2".to_string(), 2),
                ],
                asks: vec![
                    ("40010".to_string(), "0.1".to_string(), 1),
                    ("39910".to_string(), "0.2".to_string(), 2),
                ],
            },
        );
        let kraken_order = KrakenResult {
            result: kraken_data,
        };

        // convert to OrderBook format
        let coinbase_book: OrderBook = coinbase_order.into();
        let gemini_book: OrderBook = gemini_order.into();
        let kraken_order: OrderBook = kraken_order.into();

        // Merge the bids
        let mut merged_bids = coinbase_book.bids;
        merged_bids.extend(gemini_book.bids);
        merged_bids.extend(kraken_order.bids);

        // Sort bids in descending order. If this is not your sorting order, please change accordingly.
        merged_bids.sort_by(|a, b| b.price.partial_cmp(&a.price).unwrap());

        // Verify merged by comparing to expected bids
        let expected_merged_bids = vec![
            Order {
                price: 40050.0,
                quantity: 0.2,
            }, // From Gemini
            Order {
                price: 40000.0,
                quantity: 0.1,
            }, // From Coinbase
            Order {
                price: 40000.0,
                quantity: 0.1,
            }, // From Kraken
            Order {
                price: 39950.0,
                quantity: 0.2,
            }, // From Gemini
            Order {
                price: 39900.0,
                quantity: 0.2,
            }, // From Coinbase
            Order {
                price: 39900.0,
                quantity: 0.2,
            }, // From Kraken
        ];

        assert_eq!(merged_bids, expected_merged_bids, "Bids merging failed");
    }
}
