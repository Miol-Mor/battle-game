use actix::{Addr, Message};

use serde::Serialize;
use std::sync::Mutex;

use crate::websocket::Websocket;

#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct Msg(pub String);

#[derive(Debug)]
pub struct Communicator {
    addresses: Vec<Addr<Websocket>>,
}

impl Communicator {
    pub fn new(addresses: Vec<Addr<Websocket>>) -> Communicator {
        Communicator { addresses }
    }

    pub fn broadcast<T: Serialize>(&self, msg: &T) {
        self.send_message(msg, vec![]);
    }

    pub fn broadcast_everyone_but<T: Serialize>(&self, msg: &T, blacklisted: Addr<Websocket>) {
        self.send_message(msg, vec![blacklisted]);
    }

    fn send_message<T: Serialize>(&self, msg: &T, blacklist: Vec<Addr<Websocket>>) {
        for c in &self.addresses {
            if !blacklist.contains(&c) {
                c.do_send(Msg(serde_json::to_string(&msg).unwrap()));
            }
        }
    }
}
