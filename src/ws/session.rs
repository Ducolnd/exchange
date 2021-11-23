use std::time::{Duration, Instant};

use actix::prelude::*;
use actix_web_actors::ws;

use crate::ws::ws_server::Server;
use crate::ws::types::{Connect, ServerMessage};

use super::types::{Disconnect, SubscribeNotify, UnsubscribeNotify};
use super::ws_channel_messages::ChannelMessage;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);


pub struct Session {
    hb: Instant,    
    server: Addr<Server>,
    id: usize,
}

impl Actor for Session {
    type Context = ws::WebsocketContext<Self>;

    /// Method is called on actor start. We start the heartbeat process here.
    fn started(&mut self, ctx: &mut Self::Context) {
        // Start heartbeat
        self.hb(ctx);

        self.server.send(Connect {
            addr: ctx.address().recipient(),
        }).into_actor(self).then(|res, act, ctx| {
            match res {
                Ok(id) => act.id = id,
                Err(_) => ctx.stop(), // Something is wrong
            }
            fut::ready(())
        }).wait(ctx);

    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        self.server.do_send(Disconnect {
            id: self.id,
        });

        Running::Stop
    }
}

// We send messages from server to the client
impl Handler<ChannelMessage> for Session {
    type Result = ();

    fn handle(&mut self, msg: ChannelMessage, ctx: &mut Self::Context) {
        ctx.text(serde_json::to_string(&msg).unwrap());
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for Session {
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        // process websocket messages
        match msg {
            Ok(ws::Message::Text(text)) => {
                let data: ServerMessage = serde_json::from_str(&text).unwrap();

                // All the types of messages a client can send.
                match data {
                    ServerMessage::Subscribe(subscription) => {
                        self.server.do_send(SubscribeNotify {
                            id: self.id,
                            subscription,
                        });
                    },
                    ServerMessage::UnSubscribe(subscription) => {
                        self.server.do_send(UnsubscribeNotify {
                            id: self.id,
                            subscription: subscription,
                        })
                    },
                    ServerMessage::Transaction(transaction) => {
                        println!("Received transaction");
                        self.server.do_send(transaction);
                    },
                    _ => {panic!()}
                }
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

impl Session {
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
}