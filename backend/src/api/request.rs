use actix::Message;
use serde::{Deserialize, Serialize};

use super::common::Point;
use crate::api::response::Error;

pub const CMD_CLICK: &str = "click";
pub const CMD_SKIP_TURN: &str = "skip_turn";
pub const CMD_NUM_PLAYERS: &str = "num_players";

#[derive(Serialize, Deserialize, Debug, Message)]
#[rtype(result = "Option<Error>")]
pub struct Click {
    pub target: Point,
}

impl Click {
    pub fn from_str(s: &str) -> Click {
        serde_json::from_str(s).unwrap()
    }
}

#[derive(Debug, Message)]
#[rtype(result = "Option<Error>")]
pub struct SkipTurn;

#[derive(Serialize, Deserialize, Debug, Message)]
#[rtype(result = "Option<Error>")]
pub struct NumPlayers {
    pub num: usize,
}

impl NumPlayers {
    pub fn from_str(s: &str) -> NumPlayers {
        serde_json::from_str(s).unwrap()
    }
}
