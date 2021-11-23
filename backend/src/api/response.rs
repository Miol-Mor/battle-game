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
const CMD_HURT: &str = "hurt";
const CMD_DIE: &str = "die";
const CMD_UPDATE: &str = "update";
const CMD_GAME_END: &str = "end";
const CMD_CONNECTION_QUEUE: &str = "queue";

#[derive(Serialize)]
pub struct Field {
    cmd: String,
    num_x: u32,
    num_y: u32,
    field: Grid,
}

impl Field {
    pub fn new(game: &Game) -> Field {
        Field {
            cmd: CMD_FIELD.to_string(),
            num_x: game.field.num_x,
            num_y: game.field.num_y,
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
    pub fn new(target: Point, highlight_hexes: &[Point]) -> Selecting {
        Selecting {
            cmd: CMD_SELECT.to_string(),
            target,
            highlight_hexes: highlight_hexes.to_vec(),
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
    coords: Vec<Hex>,
}

impl Moving {
    pub fn new(hexes: Vec<Hex>) -> Moving {
        Moving {
            cmd: CMD_MOVE.to_string(),
            coords: hexes,
        }
    }
}

#[derive(Serialize, Debug)]
pub struct Attacking {
    cmd: String,
    from: Point,
    to: Point,
}

impl Attacking {
    pub fn new(from: Point, to: Point) -> Attacking {
        Attacking {
            cmd: CMD_ATTACK.to_string(),
            from,
            to,
        }
    }
}

#[derive(Serialize, Debug)]
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
pub struct Hurt {
    cmd: String,
    hexes: Vec<Hex>,
}

impl Hurt {
    pub fn new(hexes: Vec<Hex>) -> Hurt {
        Hurt {
            cmd: CMD_HURT.to_string(),
            hexes,
        }
    }
}

#[derive(Serialize, Debug)]
pub struct Die {
    cmd: String,
    hexes: Vec<Hex>,
}

impl Die {
    pub fn new(hexes: Vec<Hex>) -> Die {
        Die {
            cmd: CMD_DIE.to_string(),
            hexes,
        }
    }
}

#[derive(Serialize, Debug)]
pub struct Update {
    cmd: String,
    hexes: Vec<Hex>,
}

impl Update {
    pub fn new(hexes: Vec<Hex>) -> Update {
        Update {
            cmd: CMD_UPDATE.to_string(),
            hexes,
        }
    }
}

#[derive(Serialize, Debug)]
pub struct End {
    cmd: String,
    you_win: bool,
}

impl End {
    pub fn new(you_win: bool) -> End {
        End {
            cmd: CMD_GAME_END.to_string(),
            you_win,
        }
    }
}

#[derive(Serialize)]
pub struct ConnectionQueue {
    cmd: String,
    players_number: u32,
    your_number: u32,
}

impl ConnectionQueue {
    pub fn new(total_players: u32, queue_number: u32) -> ConnectionQueue {
        ConnectionQueue {
            cmd: CMD_CONNECTION_QUEUE.to_string(),
            players_number: total_players,
            your_number: queue_number,
        }
    }
}
