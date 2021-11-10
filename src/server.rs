use std::sync::mpsc::Sender;

use actix_web::{post, web, App, HttpServer, HttpResponse};

use crate::types::Transaction;


#[post("/")]
pub async fn create(transaction: web::Json<Transaction>, data: web::Data<Sender<Transaction>>) -> HttpResponse {
    data.clone().send(transaction.into_inner()).unwrap();
    HttpResponse::Ok().json("success")
}

#[actix_web::main]
pub async fn start_server(tx: Sender<Transaction>) -> std::io::Result<()> {

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