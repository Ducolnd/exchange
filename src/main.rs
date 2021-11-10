mod types;

use types::{Book, OrderType};

fn main() {
    let mut book = Book::new();
    book.new_order(OrderType::SELL, 9, 420);
    book.new_order(OrderType::BUY, 32, 400);

    book.new_order(OrderType::BUY, 46, 6);
    book.new_order(OrderType::SELL, 45, 1);

    book.new_order(OrderType::SELL, 12, 4);
    book.new_order(OrderType::BUY, 14, 23);
    book.new_order(OrderType::BUY, 12, 5);

    println!("{:?}", book.buy_book.clone().into_sorted_vec());
}