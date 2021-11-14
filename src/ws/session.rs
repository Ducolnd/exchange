use std::collections::HashMap;
use std::time::{Duration, Instant};

use actix::prelude::*;
use actix_web_actors::ws;

use crate::types::{Transaction, BuyOrder, SellOrder};
use crate::ws::wsServer::Server;
use crate::ws::types::{Connect, Disconnect};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_UPDATE_INTERVAL: Duration = Duration::from_millis(500);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);


pub struct WsConnection {
    hb: Instant,    
    server: Addr<Server>,
    id: usize,
}

impl Actor for WsConnection {
    type Context = ws::WebsocketContext<Self>;

    /// Method is called on actor start. We start the heartbeat process here.
    fn started(&mut self, ctx: &mut Self::Context) {
        // Start heartbeat
        self.hb(ctx);

        self.server.do_send(Connect {
            // Todo send data to server
        }); // Todo do something with received id

    }

    fn stopping(&mut self, ctx: &mut Self::Context) -> Running {
        self.server.send(Disconnect {

        });
        Running::Stop
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

            },
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Binary(_)) => {
                println!("Unexpected binary");
            },
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => ctx.stop(),
        }
    }
}

impl WsConnection {
    pub fn new(
        hb: Instant,
        server: Addr<Server>,
    ) -> Self {
        Self {
            hb,
            server,
            id: 0,
        }
    }

    /// Helper method that sends ping to client every second.
    fn hb(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                // heartbeat timed out
                println!("Websocket Client heartbeat failed, disconnecting!");

                // stop actor
                ctx.stop();

                // don't try to another ping
                return;
            }

            ctx.ping(b"");
        });
    }

    // fn update_client(&self, ctx: &mut <Self as Actor>::Context) {       
    //     ctx.run_interval(CLIENT_UPDATE_INTERVAL, |act, ctx| {
            
    //         if let Ok(read) = act.book.read() {
    //             let reset = WsConnection::compare_old_new(&act.buffered_state, &read.buffered_state);

    //             if !(reset.0.is_empty() && reset.1.is_empty()) {
    //                 let item = json!({
    //                     "buy": reset.0,
    //                     "sell": reset.1,
    //                 });
    
    //                 ctx.text(serde_json::to_string(&item).unwrap());

    //                 act.buffered_state = read.buffered_state.clone(); // Update
    //             }
    //         }
    //     });
    // }

    /// What has changed?
    fn compare_old_new(
        old: &(Vec<BuyOrder>, Vec<SellOrder>),
        new: &(Vec<BuyOrder>, Vec<SellOrder>),

    ) -> (HashMap<String, Vec<BuyOrder>>, HashMap<String, Vec<SellOrder>>) {
        let add_buy: Vec<_> = (new.0.clone()).into_iter().filter(|item| !old.0.clone().contains(item)).collect();
        let delete_buy: Vec<_> = (old.0.clone()).into_iter().filter(|item| !new.0.clone().contains(item)).collect();

        let add_sell: Vec<_> = (new.1.clone()).into_iter().filter(|item| !old.1.clone().contains(item)).collect();
        let delete_sell: Vec<_> = (old.1.clone()).into_iter().filter(|item| !new.1.clone().contains(item)).collect();

        let mut hash = HashMap::new();
        if add_buy.len() > 0 {
            hash.insert("add".to_string(), add_buy);
        }
        if delete_buy.len() > 0 {
            hash.insert("delete".to_string(), delete_buy);
        }

        let mut hash2 = HashMap::new();
        if add_sell.len() > 0 {
            hash2.insert("add".to_string(), add_sell);
        }
        if delete_sell.len() > 0 {
            hash2.insert("delete".to_string(), delete_sell);
        }

        (hash, hash2)
    }
}