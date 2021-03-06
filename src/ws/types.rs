use std::{collections::HashMap, iter::Product};

use actix::prelude::*;
use serde::{Deserialize, Serialize};

use crate::types::{Transaction, BuyOrder, SellOrder};

use super::ws_channel_messages::ChannelMessage;

// Messages for communicating between Actors
#[derive(Message)]
#[rtype(usize)]
pub struct Connect {
    pub addr: Recipient<ChannelMessage>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: usize,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct UnsubscribeNotify {
    pub id: usize,
    pub subscription: Subscribe,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct SubscribeNotify {
    pub id: usize,
    pub subscription: Subscribe,
}
///////////////////////////////

/// Messages client can send to the server
#[derive(Serialize, Deserialize, Message, Debug)]
#[serde(tag="type")]
#[rtype(result = "()")]
pub enum ServerMessage {
    /// Client sends new transaction
    Transaction(Transaction),
    /// Client subscribes
    Subscribe(Subscribe),
    /// Client unsubcribe
    UnSubscribe(Subscribe),
}

/// A subscribe message must be send withing a few seconds of connecting to the server.
/// This message indicates what client want to be updated on, for example
/// a Heartbeat update (every second) on the current price a product.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag="channel-type")]
pub struct Subscribe {
    pub channel: Channels,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag="channel-type")]
pub struct UnSubscribe {
    channel: Channels,
}

/// A client can subscribe to a 'channel'.
/// This is a client --> server message.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
#[serde(tag="channel-type")]
pub enum Channels {
    /// The Hearbeat channel provides most recent ticker data (bid and ask price and size)
    /// on a predetermined interval (1s)
    Heartbeat,
    /// The ticker channel provides real-time price updates every time a match happens. It will 
    /// provide the client with the latest 
    /// It batches updates in case of cascading matches, greatly reducing bandwidth requirements.
    Ticker,
    /// Keep a snapshot of the entire order book.
    Snapshot,
    /// Real-time updates on all orders and trades
    Full,
}

/// To what products a client wants to subscribe. Right now this is unimplemented and 
/// is there only one prodcts
#[derive(Serialize, Deserialize, Debug)]
pub struct Products {
    products: Vec<String>,
}