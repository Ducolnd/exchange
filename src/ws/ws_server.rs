use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::Duration;

use actix::prelude::*;
use crossbeam_channel::Sender;
use rand::prelude::{Rng, ThreadRng};

use crate::types::{Book, BuyOrder, SellOrder, Transaction};
use crate::ws::types::*;

const CLIENT_UPDATE_INTERVAL: Duration = Duration::from_millis(500);


/// Server manages updating clients. Actix Actor...
pub struct Server {
    sessions: HashMap<usize, Recipient<ClientMessages>>,
    book: Arc<RwLock<Book>>,
    book_channel: Sender<Transaction>,
    rng: ThreadRng,
    buffered_state: (Vec<BuyOrder>, Vec<SellOrder>),
}

impl Server {
    pub fn new(book: Arc<RwLock<Book>>, book_channel: Sender<Transaction>) -> Self {
        Self {
            sessions: HashMap::new(),
            book,
            book_channel,
            rng: rand::thread_rng(),
            buffered_state: (vec![], vec![]),
        }
    }

    /// Update buffered state every n millis
    fn update_clients(&mut self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(CLIENT_UPDATE_INTERVAL, |act, _| {
            if let Ok(book) = act.book.read() {
                let clone = book.buffered_state.clone();
                let compare = Server::compare_old_new(&act.buffered_state, &clone);

                if compare.0.len() > 0 || compare.1.len() > 0 {
                    act.send_all_sessions(ClientMessages::OrderBookUpdate {
                        buy: compare.0,
                        sell: compare.1,
                    });
                    act.buffered_state = book.buffered_state.clone();
                }
            }
        });
    }

    fn send_all_sessions(&self, message: ClientMessages) {
        for recipient in self.sessions.values() {
            recipient.do_send(message.clone()).unwrap();
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
}

impl Actor for Server {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.update_clients(ctx);
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

impl Handler<Disconnect> for Server {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        if self.sessions.remove(&msg.id).is_none() {
            println!("Error upon disconnect, the id did not exist");
        }
    }
}

impl Handler<RequestEntireBook> for Server {
    type Result = ();

    fn handle(&mut self, msg: RequestEntireBook, _: &mut Self::Context) -> Self::Result {
        if let Some(recipient) = self.sessions.get(&msg.id) {
            let state = self.buffered_state.clone();
            recipient
                .do_send(ClientMessages::OrderBook {
                    buy: state.0,
                    sell: state.1,
                })
                .unwrap();
        }
    }
}

impl Handler<Transaction> for Server {
    type Result = ();

    fn handle(&mut self, msg: Transaction, _: &mut Context<Self>) {
        // Send transaction to Order Book
        if let Err(res) = self.book_channel.try_send(msg) {
            println!("Something is wrong with book_channel: {:?}", res);
            return;
        };

        // Update all clients on new transaction
        self.send_all_sessions(ClientMessages::Transaction(msg));
    }
}
