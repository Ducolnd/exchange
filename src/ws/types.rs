use std::collections::HashMap;

use actix::prelude::*;
use serde::{Deserialize, Serialize};

use crate::types::{Transaction, BuyOrder, SellOrder};

// Actor types
#[derive(Message)]
#[rtype(usize)]
pub struct Connect {
    pub addr: Recipient<ClientMessages>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: usize,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct RequestEntireBook {
    pub id: usize,
}

/// Messages server can send to client 
#[derive(Serialize, Deserialize, Message, Debug, Clone)]
#[serde(tag="type")]
#[rtype(result = "()")]
pub enum ClientMessages {
    Transaction(Transaction),
    /// Entire Order Book
    OrderBook {
        buy: Vec<BuyOrder>,
        sell: Vec<SellOrder>,
    },
    /// Difference between old and new
    OrderBookUpdate { 
        buy: HashMap<String, Vec<BuyOrder>>,
        sell: HashMap<String, Vec<SellOrder>>,
    }
}

/// Messages client can send to the server
#[derive(Serialize, Deserialize, Message, Debug)]
#[serde(tag="type")]
#[rtype(result = "()")]
pub enum ServerMessage {
    /// Client feels the need to re-receive data, maybe an error occured on client side
    RequestEntireBook,
    /// Client sends new transaction
    Transaction(Transaction),
}