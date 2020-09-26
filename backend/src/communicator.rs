use actix::{Addr, Message};

use serde::Serialize;

use crate::websocket::Websocket;

#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct Msg(pub String);

pub fn broadcast<T: Serialize>(msg: &T, recipients: Vec<Addr<Websocket>>) {
    for c in recipients {
        c.do_send(Msg(serde_json::to_string(&msg).unwrap()));
    }
}
