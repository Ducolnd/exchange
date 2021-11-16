use std::time::Instant;
use std::sync::{Arc, RwLock};

use actix::Actor;
use actix_web::{App, Error, HttpRequest, HttpResponse, HttpServer, Responder, get, post, web};
use actix_cors::Cors;
use actix::prelude::Addr;
use actix_web_actors::ws;
use crossbeam_channel::Sender;

use crate::types::{Transaction, Book};
use crate::ws::session::{Session};
use crate::ws::ws_server::Server;


#[post("/")]
async fn create(transaction: web::Json<Transaction>, data: web::Data<Sender<Transaction>>) -> HttpResponse {
    data.clone().send(transaction.into_inner()).unwrap();
    HttpResponse::Ok().json("success")
}

#[get("/test")]
async fn test(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

// Entry point for the websocket route
#[get("/ws/")]
async fn ws_route(
    req: HttpRequest,
    stream: web::Payload,
    server_ref: web::Data<Addr<Server>>,
) -> Result<HttpResponse, Error> {

    println!("Req: {:?}", &req);
    
    ws::start(
        Session::new(
            Instant::now(),
                server_ref.get_ref().clone(),
        ),
        &req,
        stream,
    )
}

#[actix_web::main]
pub async fn start_server(tx: Sender<Transaction>, book: Arc<RwLock<Book>>) -> std::io::Result<()> {

    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    env_logger::init();

    let server = Server::new(book, tx.clone()).start();

    // Start api server
    HttpServer::new(move|| {
        let transactor = web::Data::new(tx.clone());
        let server_ref = web::Data::new(server.clone());


        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_header()
            .allow_any_method();

        App::new()
            .wrap(cors)
            .app_data(transactor)
            .app_data(server_ref)
            .service(create)
            .service(test)
            .service(ws_route)
        })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}