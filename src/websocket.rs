use std::collections::HashMap;
use std::time::{Duration, Instant};
use std::sync::{Arc, RwLock};

use crossbeam_channel::{Receiver, Sender};
use serde_json::json;

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
    buffered_state: (Vec<BuyOrder>, Vec<SellOrder>),
}

impl Actor for WsConnection {
    type Context = ws::WebsocketContext<Self>;

    /// Method is called on actor start. We start the heartbeat process here.
    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
        self.update_client(ctx);

        // ToDo: send entire book on first connection
        
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
    pub fn new(
        hb: Instant,
        tx: Sender<Transaction>,
        book: Arc<RwLock<Book>>,
    ) -> Self {
        Self {
            hb,
            tx,
            book,
            buffered_state: (vec![], vec![]),
        }
    }

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
                let reset = WsConnection::compare_things2(&act.buffered_state, &read.buffered_state);

                if !(reset.0.is_empty() && reset.1.is_empty()) {
                    let item = json!({
                        "buy": reset.0,
                        "sell": reset.1,
                    });

                    println!("Reset: {:?}", reset);
    
                    ctx.text(serde_json::to_string(&item).unwrap());

                    act.buffered_state = read.buffered_state.clone();
                }

            }
        });
    }

    fn compare_things2(
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

    /// Compares old order book data with new
    /// and returns a HashMap with the indexes
    /// of old data to be refreshed
    fn compare_things(
        old: &(Vec<BuyOrder>, Vec<SellOrder>),
        new: &(Vec<BuyOrder>, Vec<SellOrder>),

    ) -> (Option<HashMap<usize, BuyOrder>>, Option<HashMap<usize, SellOrder>>){
        let mut changed_buys: Option<HashMap<_,_>> = None;
        let mut changed_sells: Option<HashMap<_,_>> = None;
        
        // Buy
        let newb = &new.0;
        let oldb = &old.0;

        let newlen = newb.len();
        let oldlen = oldb.len();
        
        let mut frombuy: i16 = -1; // -1 = nothing new found
        let stop = oldlen.min(newlen);

        for i in 0..stop {
            if !(oldb[i] == newb[i]) {
                println!("Checking for {:?} and {:?}", oldb[i], newb[i]);
                frombuy = i as i16;
                break;
            }
        }

        if newlen > oldlen {
            if frombuy == -1 {
                changed_buys = Some((oldlen..newlen).zip(newb[oldlen..newlen].to_vec().into_iter()).collect());
            } 
            else {
                changed_buys = Some((frombuy as usize..newlen).zip(newb[frombuy as usize..newlen].to_vec().into_iter()).collect());
            }
        }
        
        else if newlen == oldlen {
            if frombuy == -1 {
                changed_buys = None; // Nothing's changed
            } else {
                changed_buys = Some((frombuy as usize..newlen).zip(newb[frombuy as usize..newlen].to_vec().into_iter()).collect());
            }
        }
        
        else {
            if frombuy == -1 {
                let mut a= HashMap::new();
                
                for i in newlen..oldlen {
                    a.insert(i, BuyOrder {price: 0, size: 0, timestamp: 0});
                }

                changed_buys = Some(a);

            } else {
                let mut a = (frombuy as usize..newlen).zip(newb[frombuy as usize..newlen].to_vec().into_iter()).collect::<HashMap<usize, BuyOrder>>();

                for i in newlen..oldlen {
                    a.insert(i, BuyOrder {price: 0, size: 0, timestamp: 0}).unwrap();
                }

                changed_buys = Some(a);
            }
        }
       

        // Sell
        let mut news = new.1.clone();
        let mut olds = old.1.clone();

        news.sort();
        olds.sort();

        println!("news {:?} olds {:?}", news, olds);

        let newlen = news.len();
        let oldlen = olds.len();
        
        let mut frombuy: i16 = -1; // -1 = nothing new found
        let stop = oldlen.min(newlen);

        for i in 0..stop {
            if !(news[i] == olds[i]) {
                println!("Checking for {:?} and {:?}", olds[i], news[i]);
                frombuy = i as i16;
                break;
            }
        }

        if newlen > oldlen {
            if frombuy == -1 {
                changed_sells = Some((oldlen..newlen).zip(news[oldlen..newlen].to_vec().into_iter()).collect());
                println!("ofund nothing sending all");
            } 
            else {
                changed_sells = Some((frombuy as usize..newlen).zip(news[frombuy as usize..newlen].to_vec().into_iter()).collect());
                println!("Founding something sending a part")
            }
        }
        
        else if newlen == oldlen {
            if frombuy == -1 {
                changed_sells = None; // Nothing's changed
            } else {
                changed_sells = Some((frombuy as usize..newlen).zip(news[frombuy as usize..newlen].to_vec().into_iter()).collect());
            }
        }
        
        else {
            if frombuy == -1 {
                let mut a= HashMap::new();
                
                for i in newlen..oldlen {
                    a.insert(i, SellOrder {price: 0, size: 0, timestamp: 0});
                }

                changed_sells = Some(a);
            } else {
                let mut a = (frombuy as usize..newlen).zip(news[frombuy as usize..newlen].to_vec().into_iter()).collect::<HashMap<usize, SellOrder>>();

                println!("Found new things and need to remove stuff from {}", frombuy);


                for i in newlen..oldlen {
                    a.insert(i, SellOrder {price: 0, size: 0, timestamp: 0});
                }

                changed_sells = Some(a);
            }
        }

        (changed_buys, changed_sells)
    }
}