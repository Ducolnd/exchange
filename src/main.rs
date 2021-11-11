use std::sync::{Arc, RwLock};
use std::time::Duration;
use std::{thread, time::Instant};

use crossbeam_channel::unbounded;

mod types;
mod server;
mod websocket;

use types::{Book, OrderType, Transaction, SellOrder, BuyOrder};
use server::start_server;

fn main(){
    // The channel for communicating new transactions
    let (tx_transaction, rx_ransaction) = unbounded::<Transaction>();

    let book = Book::new();
    let arc = Arc::new(RwLock::new(book));

    let write_lock = arc.clone();
    let read_lock = arc.clone();

    // This thread will run the server
    let handle = thread::spawn(move || {
        // Start the server, give it its own send channel for communicating transactions
        match start_server(tx_transaction.clone(), write_lock) {
            Err(e) => println!("An error occured: {:?}", e),
            _ => (),
        };
    });

    // The thread on which the exchange runs (Book thread) and lives
    thread::spawn(move || {


        let iter = rx_ransaction.iter();

        for element in iter {
            println!("New transaction came in: {:?}", element);

            if let Ok(mut bookarc) = read_lock.write() {
                match element.sell {
                    true => bookarc.new_order(OrderType::SELL, element.price, element.size),
                    false => bookarc.new_order(OrderType::BUY, element.price, element.size),
                };

            }
        }
    });



    handle.join().unwrap();
}