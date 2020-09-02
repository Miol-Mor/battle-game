use serde::Serialize;

use super::common::Point;

const CMD_MOVE: &str = "moving";
const CMD_ATTACK: &str = "attacking";
pub const CMD_TURN: &str = "turn";
const CMD_ERROR: &str = "error";

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

#[derive(Serialize, Debug)]
pub struct ResponseError {
    cmd: String,
    message: String,
}

impl ResponseError {
    pub fn new(message: String) -> ResponseError {
        ResponseError {
            cmd: CMD_ERROR.to_string(),
            message,
        }
    }
}

#[derive(Serialize)]
pub struct Changes {}
