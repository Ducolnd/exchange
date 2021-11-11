use std::sync::mpsc::Sender;
use std::time::Instant;

use actix_web::{App, Error, HttpRequest, HttpResponse, HttpServer, Responder, get, post, web};
use actix_cors::Cors;
use actix_web_actors::ws;

use crate::types::Transaction;
use crate::websocket::{WsConnection};


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
    channel: web::Data<Sender<Transaction>>,
) -> Result<HttpResponse, Error> {
    println!("Req: {:?}", &req);
    ws::start(
        WsConnection {
            hb: Instant::now(),
            tx: channel.get_ref().clone(),
        },
        &req,
        stream,
    )
}

#[actix_web::main]
pub async fn start_server(tx: Sender<Transaction>) -> std::io::Result<()> {

    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    env_logger::init();

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
            .service(test)
            .service(ws_route)
        })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}