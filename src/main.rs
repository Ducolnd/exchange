use std::thread;
use std::sync::{mpsc::channel, mpsc::Sender};

use actix_web::{post, web, App, HttpServer, HttpResponse};

mod types;
use types::{Book, OrderType, Transaction};


#[post("/")]
pub async fn create(transaction: web::Json<Transaction>, data: web::Data<Sender<Transaction>>) -> HttpResponse {
    data.clone().send(transaction.into_inner()).unwrap();
    HttpResponse::Ok().json("success")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
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

    // Start api server
    HttpServer::new(move|| {
        let data = web::Data::new(tx.clone());
        
        App::new()
            .app_data(data)
            .service(create)
        })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}