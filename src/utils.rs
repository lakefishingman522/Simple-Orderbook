use crate::order_book::Order;

// We can define functions to calculate the price.
// The basic idea is to walk down the order book, summing up the total
// quantity until it is more than 10, then calculate the price based on the
// quantity and price of each order.
pub fn calculate_price(order_book: &mut [Order], quantity: f64) -> f64 {
    let mut total_quantity = 0.0;
    let mut total_price = 0.0;

    // order_book.sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap());

    for order in order_book {
        if total_quantity + order.quantity >= quantity {
            let leftover = (total_quantity + order.quantity) - quantity;
            total_price += (order.quantity - leftover) * order.price;
            break;
        } else {
            total_price += order.price * order.quantity;
            total_quantity += order.quantity;
        }
    }

    total_price
}
