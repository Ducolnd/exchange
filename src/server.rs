use std::sync::mpsc::Sender;

use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, get, post, web};
use actix_cors::Cors;

use crate::types::Transaction;


#[post("/")]
async fn create(transaction: web::Json<Transaction>, data: web::Data<Sender<Transaction>>) -> HttpResponse {
    data.clone().send(transaction.into_inner()).unwrap();
    HttpResponse::Ok().json("success")
}

#[get("/test")]
async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

#[actix_web::main]
pub async fn start_server(tx: Sender<Transaction>) -> std::io::Result<()> {

    // Start api server
    HttpServer::new(move|| {
        let data = web::Data::new(tx.clone());
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_header()
            .allow_any_method();

        App::new()
            .wrap(cors)
            .app_data(data)
            .service(create)
            .service(greet)
        })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}