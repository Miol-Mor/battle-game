use serde::Serialize;

use crate::api::common::Point;
use crate::game::Game;
use crate::game_objects::{grid::Grid, hex::Hex};

const CMD_FIELD: &str = "field";
const CMD_STATE: &str = "state";
const CMD_SELECT: &str = "selecting";
const CMD_DESELECT: &str = "deselecting";
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
pub struct Selecting {
    cmd: String,
    target: Point,
    highlight_hexes: Vec<Point>,
}

impl Selecting {
    pub fn new(target: Point, highlight_hexes: &Vec<Point>) -> Selecting {
        Selecting {
            cmd: CMD_SELECT.to_string(),
            target,
            highlight_hexes: highlight_hexes.clone(),
        }
    }
}

#[derive(Serialize)]
pub struct Deselecting {
    cmd: String,
    target: Point,
}

impl Deselecting {
    pub fn new(target: Point) -> Deselecting {
        Deselecting {
            cmd: CMD_DESELECT.to_string(),
            target,
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

#[derive(Serialize)]
pub struct State {
    cmd: String,
    state: String,
}

impl State {
    pub fn new(state: String) -> State {
        State {
            cmd: CMD_STATE.to_string(),
            state,
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
