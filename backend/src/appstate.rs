use actix::Addr;

use std::sync::Mutex;

use crate::websocket::Websocket;
use crate::game;

#[derive(Debug)]
pub struct AppState {
    pub clients: Mutex<Vec<Addr<Websocket>>>,
    pub game: Mutex<game::Game>,
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            clients: Mutex::new(vec![]),
            game: Mutex::new(game::Game::new(0, 0)),
        }
    }
}
