use std::sync::{Arc, RwLock};
use std::time::Duration;
use std::{thread, time::Instant};
use actix::Actor;
use rand::Rng;

use crossbeam_channel::unbounded;

mod types;
mod server;
mod ws;

use types::{Book, OrderType, Transaction};
use ws::ws_server::Server;
use server::start_server;

fn main(){
    // The channel for communicating new transactions
    let (tx_transaction, rx_ransaction) = unbounded::<Transaction>();

    let book = Book::new();
    let arc = Arc::new(RwLock::new(book));

    let write_lock = arc.clone();
    let read_lock = arc.clone();

    // This thread will run the server
    let tx_server = tx_transaction.clone();
    let server = Server::new(write_lock, tx_server).start();

    let handle = thread::spawn(move || {
        // Start the server, give it its own send channel for communicating transactions
        match start_server(server.clone()) {
            Err(e) => println!("An error occured: {:?}", e),
            _ => (),
        };
        println!("stopped server");
    });

    // Thread which makes test orders
    for _ in 0..0 {
        let tx = tx_transaction.clone();
        thread::spawn(move || {
            let mut rng = rand::thread_rng();
            let mut earlier = Instant::now();

            thread::sleep(Duration::from_millis(rng.gen_range(0..400)));

            loop {
                if Instant::now().duration_since(earlier) > Duration::from_millis(rng.gen_range(1200..2000)) {
                    let sell_bool = rand::random();
                    tx.send(Transaction {
                        sell: sell_bool,
                        price: (if sell_bool {rng.gen_range(95..110)} else {rng.gen_range(90..105)}) * 10e9 as u64,
                        size: rng.gen_range(1..30)
                    }).unwrap();

                    earlier = Instant::now();
                }
            }
        });
    }

    // The thread on which the exchange runs (Book thread) and lives
    thread::spawn(move || {
        let iter = rx_ransaction.iter();

        let mut earlier = Instant::now();
        for element in iter {
            // println!("New transaction came in: {:?}", element);

            if let Ok(mut bookarc) = read_lock.write() {
                match element.sell {
                    true => bookarc.new_order(OrderType::SELL, element.price, element.size),
                    false => bookarc.new_order(OrderType::BUY, element.price, element.size),
                };

                if Instant::now().duration_since(earlier) > Duration::from_millis(100) {
                    earlier = Instant::now();
    
                    bookarc.update_state(); 
                    // println!("updated state");
                }
            }
        }
    });



    handle.join().unwrap();
}