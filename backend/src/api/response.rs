use serde::Serialize;

use crate::api::common::Point;
use crate::game::Game;
use crate::game_objects::{grid::Grid, hex::Hex};

const CMD_FIELD: &str = "field";
const CMD_MOVE: &str = "moving";
const CMD_ATTACK: &str = "attacking";
const CMD_ERROR: &str = "error";

#[derive(Serialize)]
pub struct Field {
    cmd: String,
    num_x: u32,
    num_y: u32,
    field: Grid,
}

impl Field {
    pub fn new(game: &Game, num_x: u32, num_y: u32) -> Field {
        Field {
            cmd: CMD_FIELD.to_string(),
            num_x,
            num_y,
            field: game.field.clone(),
        }
    }
}

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
    changes: Changes,
}

impl Attacking {
    pub fn new(from: Point, to: Point, hurt: Vec<Hex>, die: Vec<Hex>) -> Attacking {
        Attacking {
            cmd: CMD_ATTACK.to_string(),
            from,
            to,
            changes: Changes { hurt, die },
        }
    }
}

#[derive(Serialize, Debug)]
pub struct Error {
    cmd: String,
    message: String,
}

impl Error {
    pub fn new(message: String) -> Error {
        Error {
            cmd: CMD_ERROR.to_string(),
            message,
        }
    }
}

#[derive(Serialize, Debug)]
pub struct Changes {
    hurt: Vec<Hex>,
    die: Vec<Hex>,
}
