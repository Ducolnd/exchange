use std::{collections::HashMap, iter::Product};

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
    },
    /// An error with error message
    Error {message: String},
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
    Subscribe(Subscribe),
}

/// A subscribe message must be send withing a few seconds of connecting to the server.
/// This message indicates what client want to be updated on, for example
/// a Heartbeat update (every second) on the current price a product.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag="channel-type")]
pub struct Subscribe {
    channel: Channels,
}

/// A client can subscribe to a 'channel'.
/// This is a client --> server message.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag="channel-type")]
pub enum Channels {
    /// The Hearbeat channel provides most recent ticker data (bid and ask price and size)
    /// on a predetermined interval (1s)
    Heartbeat(Products),
    /// The ticker channel provides real-time price updates every time a match happens. It will 
    /// provide the client with the latest 
    /// It batches updates in case of cascading matches, greatly reducing bandwidth requirements.
    Ticker(Products),
    /// Keep a snapshot of the entire order book.
    Snapshot(Products),
    /// Real-time updates on all orders and trades
    Full(Products),
}

/// To what products a client wants to subscribe. Right now this is unimplemented and 
/// is there only one prodcts
#[derive(Serialize, Deserialize, Debug)]
pub struct Products {
    products: Vec<String>,
}