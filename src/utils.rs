use std::collections::HashMap;

use crate::order_book::Order;

// We can define functions to calculate the price.
// The basic idea is to walk down the order book, summing up the total
// quantity until it is more than 10, then calculate the price based on the
// quantity and price of each order.
pub fn calculate_price(order_book: &mut [Order], quantity: f64) -> f64 {
    let mut total_quantity = 0.0;
    let mut total_price = 0.0;

    let mut data: HashMap<String, f64> = HashMap::new();

    let mut worst_price: HashMap<String, f64> = HashMap::new();

    data.insert("coinbase".to_string(), 0.0);
    data.insert("gemini".to_string(), 0.0);
    data.insert("kraken".to_string(), 0.0);

    // order_book.sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap());

    for order in order_book {
        worst_price.insert(order.exchange_name.clone(), order.price);

        if total_quantity + order.quantity >= quantity {
            let leftover = (total_quantity + order.quantity) - quantity;
            total_price += (order.quantity - leftover) * order.price;

            let tmp = data.get(&order.exchange_name).unwrap();
            data.insert(order.exchange_name.clone(), tmp + order.quantity - leftover);

            break;
        } else {
            let tmp = data.get(&order.exchange_name).unwrap();

            data.insert(order.exchange_name.clone(), tmp + order.quantity);

            total_price += order.price * order.quantity;
            total_quantity += order.quantity;
        }
    }

    for (exchange, value) in worst_price.iter() {
        println!("worst price for exchange {} : {}", *exchange, *value);
    }

    for (exchange, quantity) in data.iter() {
        println!("quantity for exchange {} : {}", *exchange, *quantity);
    }

    total_price
}
