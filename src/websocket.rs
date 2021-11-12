use std::time::{Duration, Instant};
use std::sync::{Arc, RwLock};

use crossbeam_channel::{Receiver, Sender};

use actix::prelude::*;
use actix_web_actors::ws;

use crate::types::{Transaction, BuyOrder, SellOrder, Book};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_UPDATE_INTERVAL: Duration = Duration::from_millis(1000);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);


pub struct WsConnection {
    /// Client must send ping at least once per 10 seconds (CLIENT_TIMEOUT),
    /// otherwise we drop connection.
    pub hb: Instant,
    pub tx: Sender<Transaction>,
    pub book: Arc<RwLock<Book>>,
}

impl Actor for WsConnection {
    type Context = ws::WebsocketContext<Self>;

    /// Method is called on actor start. We start the heartbeat process here.
    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
        self.update_client(ctx);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsConnection {
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        // process websocket messages
        match msg {
            Ok(ws::Message::Text(text)) => {
                // Parse json
                let transaction: Transaction = serde_json::from_str(&text).unwrap();

                // Send transaction to Book thread
                self.tx.send(transaction).unwrap();
                println!("WS: {:?}", text);
            },
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => ctx.stop(),
        }
    }
}

impl WsConnection {
    /// helper method that sends ping to client every second.
    fn hb(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                // heartbeat timed out
                println!("Websocket Client heartbeat failed, disconnecting!");

                // stop actor
                ctx.stop();

                // don't try to send a ping
                return;
            }

            ctx.ping(b"");
        });
    }

    fn update_client(&self, ctx: &mut <Self as Actor>::Context) {       
        ctx.run_interval(CLIENT_UPDATE_INTERVAL, |act, ctx| {
            
            if let Ok(read) = act.book.read() {
                let vec = read.get_vec();

                let buy = serde_json::to_string(&vec.0).unwrap();
                let sell = serde_json::to_string(&vec.1).unwrap();

                let data = format!("[{:?},{:?}]", buy, sell);

                ctx.text(data);
            }

            ctx.ping(b"");
        });
    }
}