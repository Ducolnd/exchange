use actix::Message;
use serde::{Deserialize, Serialize};

/// Server updates client on subscribed channels.
/// This is a server --> client message.
#[derive(Serialize, Deserialize, Debug, Message)]
#[serde(tag="type")]
#[rtype(result = "()")]
pub enum ChannelMessage {
    /// An error occured
    Error {message: String},
    
    // Both level 1 updates
    Heartbeat(Heartbeat),
    Ticker(Ticker),

    // Level 2 updates
    Snapshot(Snapshot),
    SnapshotUpdate(SnapshotUpdate),

    // Level 3 updates: ToDo
    Match(Match),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Heartbeat {
    best_ask: f64,
    best_bid: f64,
    best_ask_size: f64,
    best_bid_size: f64,
    time: String,
    product: String,
}

pub type BookValue = (f64, f64);

#[derive(Serialize, Deserialize, Debug)]
pub struct Snapshot {
    time: String,
    product: String,
    bids: Vec<BookValue>,
    asks: Vec<BookValue>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SnapshotUpdate {
    time: String,
    product: String,
    buy_changes: Vec<BookValue>,
    sell_changes: Vec<BookValue>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Ticker {
    time: String,
    product: String,
    price: f64,
    side: String,
    best_ask: f64,
    best_bid: f64,
}

/// A trade occurred between two orders.
#[derive(Serialize, Deserialize, Debug)]
pub struct Match {
    size: f64,
    price: f64,
    side: String,
    product: String,
    time: String,
}