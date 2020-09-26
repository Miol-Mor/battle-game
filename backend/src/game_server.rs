use actix::{Actor, Addr, Context, Handler};

use serde::Serialize;
use tracing::instrument;

use crate::communicator;
use crate::websocket::Websocket;

use crate::api;
use crate::api::inner;
use crate::api::request::{Attack, Move};
use crate::api::response::{Attacking, Moving};
use crate::game::Game;
use crate::game_objects::hex_objects::content::Content;
use crate::game_objects::hex_objects::wall::Wall;
use crate::game_objects::unit::Unit;

#[derive(Debug)]
pub struct GameServer {
    pub clients: Vec<Addr<Websocket>>,
    pub game: Game,
    pub current_player: usize,
}

impl GameServer {
    pub fn check_player_turn(&self, addr: &Addr<Websocket>) -> bool {
        self.clients[self.current_player] == *addr
    }
}

impl Actor for GameServer {
    type Context = Context<Self>;
}

impl Handler<inner::Request<Move>> for GameServer {
    type Result = ();

    fn handle(&mut self, message: inner::Request<Move>, _: &mut Self::Context) -> Self::Result {
        debug!("Handle move");
        if !self.check_player_turn(&message.sender) {
            debug!("Error: wrong player move");
            return;
        }
        let message = message.payload;
        match self.game.move_unit(message.from, message.to) {
            Ok(path) => {
                let message = Moving::new(path);
                self.broadcast(&message);
            }
            Err(error) => {
                error!("{:?}", error.wrap_err("move handle error"));
                self.send_error(api::request::CMD_MOVE.to_string());
            }
        }
    }
}

impl Handler<inner::Request<Attack>> for GameServer {
    type Result = ();

    fn handle(&mut self, message: inner::Request<Attack>, _: &mut Self::Context) -> Self::Result {
        debug!("Handle attack");
        if !self.check_player_turn(&message.sender) {
            debug!("Error: wrong player attack");
            return;
        }
        let message = message.payload;
        match self.game.attack(message.from.clone(), message.to.clone()) {
            Ok((hurt, die)) => {
                let message = Attacking::new(message.from, message.to, hurt, die);
                self.broadcast(&message);
                self.next_turn();
            }
            Err(error) => {
                error!("{:?}", error.wrap_err("attack handle error"));
                self.send_error(api::request::CMD_ATTACK.to_string());
            }
        }
    }
}

impl Handler<inner::NewClient> for GameServer {
    type Result = ();

    fn handle(&mut self, client: inner::NewClient, _: &mut Self::Context) -> Self::Result {
        self.clients.push(client.address);

        if self.clients.len() == 2 {
            self.new_game();
        }
    }
}

impl GameServer {
    pub fn new() -> GameServer {
        GameServer {
            clients: vec![],
            game: Game::new(0, 0),
            current_player: 0,
        }
    }

    pub fn next_turn(&mut self) {
        self.change_player();
        self.send_turn();
        debug!("Game state: {:?}", self.game);
    }

    pub fn broadcast<T: Serialize>(&self, msg: &T) {
        communicator::broadcast(msg, self.clients.clone())
    }

    fn change_player(&mut self) {
        self.current_player += 1;
        self.current_player %= 2;
    }

    fn send_turn(&self) {
        let msg = api::common::Message::new(api::response::CMD_TURN);
        communicator::broadcast(&msg, vec![self.clients[self.current_player].clone()]);
    }

    fn send_error(&self, error_message: String) {
        let error = api::response::Error::new(error_message);
        communicator::broadcast(&error, vec![self.clients[self.current_player].clone()]);
    }

    pub fn new_game(&mut self) {
        let mut game = Game::new(4, 3);
        let unit0 = Unit {
            player: 0,
            hp: 10,
            damage: [2, 5],
            speed: 1,
        };
        let unit1 = Unit {
            player: 1,
            hp: 7,
            damage: [4, 6],
            speed: 2,
        };
        let wall = Wall {};

        match game.set_unit(0, 0, Some(unit0)) {
            Ok(_) => {}
            Err(error) => debug!("{:?}", error),
        }
        match game.set_unit(3, 2, Some(unit1)) {
            Ok(_) => {}
            Err(error) => debug!("{:?}", error),
        }

        match game.set_content(1, 1, Some(Content::Wall(wall.clone()))) {
            Ok(_) => {}
            Err(error) => debug!("{:?}", error),
        }
        match game.set_content(2, 2, Some(Content::Wall(wall.clone()))) {
            Ok(_) => {}
            Err(error) => debug!("{:?}", error),
        }

        self.broadcast(&game);
        self.send_turn();
        self.game = game;
    }
}
