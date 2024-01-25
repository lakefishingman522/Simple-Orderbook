#[cfg(test)]
use crate::order_book::*;
use crate::utils::calculate_price;

mod tests {
    use super::*;

    #[test]
    fn test_sorting_asks() {
        let mut merged_asks = vec![
            Order {
                price: 2.0,
                quantity: 7.0,
            },
            Order {
                price: 1.0,
                quantity: 5.0,
            },
            Order {
                price: 3.0,
                quantity: 6.0,
            },
        ];

        merged_asks.sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap());

        assert_eq!(merged_asks[0].price, 1.0);
        assert_eq!(merged_asks[1].price, 2.0);
        assert_eq!(merged_asks[2].price, 3.0);
    }

    #[test]
    fn test_sorting_bids() {
        let mut merged_bids = vec![
            Order {
                price: 2.0,
                quantity: 7.0,
            },
            Order {
                price: 1.0,
                quantity: 5.0,
            },
            Order {
                price: 3.0,
                quantity: 6.0,
            },
        ];

        merged_bids.sort_by(|a, b| b.price.partial_cmp(&a.price).unwrap());

        assert_eq!(merged_bids[0].price, 3.0);
        assert_eq!(merged_bids[1].price, 2.0);
        assert_eq!(merged_bids[2].price, 1.0);
    }

    #[test]
    fn test_calculate_price() {
        let mut orders = vec![
            Order {
                price: 1.0,
                quantity: 5.1,
            },
            Order {
                price: 2.0,
                quantity: 7.1,
            },
        ];

        let result = calculate_price(&mut orders, 10.0);

        // Assuming `calculate_price` returns a total price
        assert_eq!(result, 14.9);
    }

    #[tokio::test]
    async fn test_calculate_price_sell() {
        let mut order_book = OrderBook {
            // sequence: 0,
            bids: vec![
                Order {
                    price: 54.0,
                    quantity: 5.0,
                },
                Order {
                    price: 52.0,
                    quantity: 6.0,
                },
            ],
            asks: vec![],
        };

        // Adding sort order_book by price descending for bids
        order_book
            .bids
            .sort_by(|a, b| b.price.partial_cmp(&a.price).unwrap());

        let selling_price = calculate_price(&mut order_book.bids[..], 10.0);
        assert_eq!(selling_price, 530.0);
    }

    #[tokio::test]
    async fn test_calculate_price_buy() {
        let mut order_book = OrderBook {
            bids: vec![],
            asks: vec![
                Order {
                    price: 54.0,
                    quantity: 5.0,
                },
                Order {
                    price: 52.0,
                    quantity: 6.0,
                },
            ],
        };

        // Adding sort order_book by price ascending for asks
        order_book
            .asks
            .sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap());

        let buying_price = calculate_price(&mut order_book.asks[..], 10.0);
        assert_eq!(buying_price, 528.0);
    }
}
