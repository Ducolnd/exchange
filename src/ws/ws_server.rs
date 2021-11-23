use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::sync::{Arc, RwLock};
use std::time::Duration;

use actix::prelude::*;
use crossbeam_channel::Sender;
use rand::prelude::{Rng, ThreadRng};

use crate::types::{Book, BuyOrder, SellOrder, Transaction};
use crate::ws::types::*;

use super::ws_channel_messages::ChannelMessage;

const CLIENT_UPDATE_INTERVAL: Duration = Duration::from_millis(500);


enum ChannelSelector {
    Hearbeat,
    Ticker,
    Snapshot,
    Full,
}

/// Server manages updating clients. Actix Actor...
pub struct Server {
    book: Book,
    rng: ThreadRng,

    sessions: HashMap<usize, Recipient<ChannelMessage>>,
    subscriptions: HashMap<ChannelSelector, HashSet<usize>>,
}

impl Server {
    pub fn new() -> Self {
        Self {
            book: Book::new(),
            rng: rand::thread_rng(),
            sessions: HashMap::new(),
            subscriptions: HashMap::new(),
        }
    }
}

impl Actor for Server {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        // self.update_clients(ctx);
    }
}

// Handle incoming messages from Session
impl Handler<Connect> for Server {
    type Result = usize;

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        println!("New client");

        let id = self.rng.gen::<usize>();
        self.sessions.insert(id, msg.addr);

        id
    }
}

// Handle incoming messages from Session
impl Handler<Disconnect> for Server {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) -> Self::Result {
        self.sessions.remove(&msg.id);
    }
}

impl Handler<SubscribeNotify> for Server {
    type Result = ();

    fn handle(&mut self, msg: SubscribeNotify, ctx: &mut Self::Context) -> Self::Result {
        
    }
}

impl Handler<UnsubscribeNotify> for Server {
    type Result = ();

    fn handle(&mut self, msg: UnsubscribeNotify, _: &mut Context<Self>) {
        
    }
}

impl Handler<Transaction> for Server {
    type Result = ();

    fn handle(&mut self, msg: Transaction, _: &mut Context<Self>) {
        // Send transaction to Order Book
        
    }
}

/// What has changed?
fn compare_old_new(
    old: &(Vec<BuyOrder>, Vec<SellOrder>),
    new: &(Vec<BuyOrder>, Vec<SellOrder>),
) -> (
    HashMap<String, Vec<BuyOrder>>,
    HashMap<String, Vec<SellOrder>>,
) {
    let add_buy: Vec<_> = (new.0.clone())
        .into_iter()
        .filter(|item| !old.0.clone().contains(item))
        .collect();
    let delete_buy: Vec<_> = (old.0.clone())
        .into_iter()
        .filter(|item| !new.0.clone().contains(item))
        .collect();

    let add_sell: Vec<_> = (new.1.clone())
        .into_iter()
        .filter(|item| !old.1.clone().contains(item))
        .collect();
    let delete_sell: Vec<_> = (old.1.clone())
        .into_iter()
        .filter(|item| !new.1.clone().contains(item))
        .collect();

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