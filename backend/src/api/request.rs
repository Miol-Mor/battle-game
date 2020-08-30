use serde::Deserialize;

use super::common::Point;

pub const CMD_MOVE: &str = "move";
pub const CMD_ATTACK: &str = "attack";

#[derive(Deserialize, Debug)]
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
