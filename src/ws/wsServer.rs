use std::cell::Ref;
use std::sync::{Arc, RwLock};

use actix::prelude::*;
use rand::Rng;

use crate::ws::types::*;
use crate::types::Transaction;
use crate::types::Book;

/// Server manages updating clients. Actix Actor...
pub struct Server {
    book: Arc<RwLock<Book>>,
}

impl Server {
    pub fn new(book_arc: Arc<RwLock<Book>>) -> Self {
        Self {
            book: book_arc,
        }
    }
}

impl Actor for Server {
    type Context = Context<Self>;
}

impl Handler<Connect> for Server {
    type Result = ();

    fn handle(&mut self, msg: Connect, ctx: &mut Context<Self>) {

    }
}

impl Handler<Disconnect> for Server {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, ctx: &mut Context<Self>) {

    }
}

impl Handler<Transaction> for Server {
    type Result = ();

    fn handle(&mut self, msg: Transaction, ctx: &mut Context<Self>) {

    }
}

impl Handler<Refresh> for Server {
    type Result = ();

    fn handle(&mut self, msg: Refresh, ctx: &mut Context<Self>) {

    }
}