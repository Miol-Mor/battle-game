use actix::{Addr, Message};
use serde::{Deserialize, Serialize};

use super::common::Point;
use crate::api::response::ResponseError;
use crate::websocket::Websocket;

pub const CMD_MOVE: &str = "move";
pub const CMD_ATTACK: &str = "attack";

#[derive(Serialize, Deserialize, Debug, Message)]
#[rtype(result = "Option<ResponseError>")]
pub struct Move {
    pub from: Point,
    pub to: Point,
}

impl Move {
    pub fn from_str(s: &str) -> Move {
        serde_json::from_str(s).unwrap()
    }
}

#[derive(Deserialize, Debug)]
pub struct Attack {
    pub from: Point,
    pub to: Point,
}

impl Attack {
    pub fn from_str(s: &str) -> Attack {
        serde_json::from_str(s).unwrap()
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
