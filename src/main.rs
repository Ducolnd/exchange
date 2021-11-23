use std::sync::{Arc, RwLock};
use std::time::Duration;
use std::{thread, time::Instant};
use rand::Rng;

use crossbeam_channel::unbounded;

mod types;
mod server;
mod ws;

use types::{Book, OrderType, Transaction};
use server::start_server;

fn main(){
    // This thread will run the server
    let handle = thread::spawn(move || {
        // Start the server, give it its own send channel for communicating transactions
        match start_server() {
            Err(e) => println!("An error occured: {:?}", e),
            _ => (),
        };
    });

    handle.join().unwrap();
}