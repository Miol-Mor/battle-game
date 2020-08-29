use actix::Addr;

use serde::Serialize;
use std::sync::Mutex;

use crate::api;
use crate::communicator;
use crate::game;
use crate::websocket::Websocket;

#[derive(Debug)]
pub struct AppState {
    pub clients: Mutex<Vec<Addr<Websocket>>>,
    pub game: Mutex<game::Game>,
    pub current_player: Mutex<u32>,
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            clients: Mutex::new(vec![]),
            game: Mutex::new(game::Game::new(0, 0)),
            current_player: Mutex::new(0),
        }
    }

    pub fn next_turn(&self) {
        self.change_player();
        self.send_turn();
    }

    pub fn broadcast<T: Serialize>(&self, msg: &T) {
        let communicator = communicator::Communicator::new(self.clients.lock().unwrap().clone());

        communicator.broadcast(msg)
    }

    fn change_player(&self) {
        *self.current_player.lock().unwrap() += 1;
        *self.current_player.lock().unwrap() %= 2;
    }

    fn send_turn(&self) {
        let msg = api::common::Message::new(api::response::CMD_TURN);
        let communicator = communicator::Communicator::new(self.clients.lock().unwrap().clone());

        communicator.broadcast_everyone_but(
            &msg,
            self.clients.lock().unwrap()[*self.current_player.lock().unwrap() as usize].clone(),
        );
    }
}
