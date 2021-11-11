use std::thread;
use std::sync::mpsc::channel;

mod types;
mod server;
mod websocket;

use types::{Book, OrderType, Transaction};
use server::start_server;

fn main(){
    // The channel for communicating new transactions
    let (tx, rx) = channel::<Transaction>();

    // The thread on which the exchange runs
    thread::spawn(move|| {
        let mut book = Book::new();


        let iter = rx.iter();
        for element in iter {
            println!("New transaction came in: {:?}", element);

            match element.sell {
                true => book.new_order(OrderType::SELL, element.price, element.size),
                false => book.new_order(OrderType::BUY, element.price, element.size),
            };
        }
    });

    // This thread will run the server
    let handle = thread::spawn(move || {
        // Start the server, give it its own send channel for communicating transactions
        match start_server(tx.clone()) {
            Err(e) => println!("An error occured: {:?}", e),
            _ => (),
        };
    });

    handle.join().unwrap();
}