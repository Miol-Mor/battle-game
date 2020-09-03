use crate::websocket::Websocket;
use actix::{Addr, Message};

#[derive(Debug, Message)]
#[rtype(result = "()")]
pub struct Request<T> {
    pub sender: Addr<Websocket>,
    pub payload: T,
}

impl<T> Request<T> {
    pub fn new(addr: Addr<Websocket>, payload: T) -> Request<T> {
        Request {
            sender: addr,
            payload,
        }
    }
}

#[derive(Debug, Message)]
#[rtype(result = "()")]
pub struct NewClient {
    pub address: Addr<Websocket>,
}

impl NewClient {
    pub fn new(address: Addr<Websocket>) -> NewClient {
        NewClient { address }
    }
}
