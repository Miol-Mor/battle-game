use actix::Addr;

use serde::Serialize;
use std::sync::Mutex;

use crate::api;
use crate::communicator;
use crate::websocket::Websocket;

use crate::game::Game;
use crate::game_objects::hex_objects::content::Content;
use crate::game_objects::hex_objects::wall::Wall;
use crate::game_objects::unit::Unit;

#[derive(Debug)]
pub struct AppState {
    pub clients: Mutex<Vec<Addr<Websocket>>>,
    pub game: Mutex<Game>,
    pub current_player: Mutex<u32>,
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            clients: Mutex::new(vec![]),
            game: Mutex::new(Game::new(0, 0)),
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

    pub fn process_message(&self, text: String) {
        let message = api::common::Message::from_str(&text);
        match message.cmd.as_str() {
            api::request::CMD_MOVE => {
                self.process_move(api::request::Move::from_str(&text));
            }
            api::request::CMD_ATTACK => {
                self.process_attack(api::request::Attack::from_str(&text));
            }
            _ => {
                debug!("Unknown command: {}", message.cmd);
            }
        }
    }

    fn process_move(&self, data: api::request::Move) {
        // let mut game = self.game.lock().unwrap();
        let response = api::response::Moving::new(vec![data.from, data.to]);
        self.broadcast(&response);
    }

    fn process_attack(&self, data: api::request::Attack) {
        let response = api::response::Attacking::new(data.from, data.to);
        self.broadcast(&response);

        // After attack turn ends
        self.next_turn();
    }

    pub fn new_game(&self) {
        let mut game = Game::new(4, 3);
        let unit0 = Unit {
            player: 0,
            hp: 10,
            damage: [2, 5],
            speed: 1,
        };
        let unit1 = Unit {
            player: 1,
            hp: 1,
            damage: [4, 6],
            speed: 2,
        };
        let wall = Wall {};

        match game.set_unit(0, 0, unit0) {
            Ok(_) => {}
            Err(error) => debug!("{:?}", error),
        }
        match game.set_unit(3, 2, unit1) {
            Ok(_) => {}
            Err(error) => debug!("{:?}", error),
        }

        match game.set_content(1, 1, Content::Wall(wall.clone())) {
            Ok(_) => {}
            Err(error) => debug!("{:?}", error),
        }
        match game.set_content(2, 2, Content::Wall(wall.clone())) {
            Ok(_) => {}
            Err(error) => debug!("{:?}", error),
        }

        self.broadcast(&game);
        self.next_turn();
        *self.game.lock().unwrap() = game;
    }
}
