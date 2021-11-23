use std::thread;

mod types;
mod server;
mod ws;

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