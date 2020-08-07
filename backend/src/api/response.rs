use serde::Serialize;

use super::common::Point;

const CMD_MOVE: &str = "moving";
const CMD_ATTACK: &str = "attacking";
pub const CMD_TURN: &str = "turn";

#[derive(Serialize)]
pub struct Moving {
    cmd: String,
    coords: Vec<Point>,
}

impl Moving {
    pub fn new(points: Vec<Point>) -> Moving {
        Moving {
            cmd: CMD_MOVE.to_string(),
            coords: points,
        }
    }
}

#[derive(Serialize)]
pub struct Attacking {
    cmd: String,
    from: Point,
    to: Point,
    changes: Option<Changes>,
}

impl Attacking {
    pub fn new(from: Point, to: Point) -> Attacking {
        Attacking {
            cmd: CMD_ATTACK.to_string(),
            from,
            to,
            changes: None,
        }
    }
}

#[derive(Serialize)]
pub struct Changes {}
