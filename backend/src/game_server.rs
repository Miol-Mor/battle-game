use actix::{Actor, Addr, Context, Handler};

use serde::Serialize;

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
    pub current_player: u8,
}

impl Actor for GameServer {
    type Context = Context<Self>;
}

impl Handler<inner::Request<Move>> for GameServer {
    type Result = ();

    fn handle(&mut self, message: inner::Request<Move>, _: &mut Self::Context) -> Self::Result {
        debug!("Appstate process move");
        let message = message.payload;
        match self.game.move_unit(message.from, message.to) {
            Ok(path) => {
                let message = Moving::new(path);
                self.broadcast(&message);
            }
            Err(_) => unimplemented!(),
        }
    }
}

impl Handler<inner::Request<Attack>> for GameServer {
    type Result = ();

    fn handle(&mut self, message: inner::Request<Attack>, _: &mut Self::Context) -> Self::Result {
        debug!("Handle attack");
        let message = message.payload;
        match self.game.attack(message.from.clone(), message.to.clone()) {
            Ok((hurt, die)) => {
                let message = Attacking::new(message.from, message.to, hurt, die);
                self.broadcast(&message);
                self.next_turn();
            }
            Err(_) => unimplemented!(),
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
        let communicator = communicator::Communicator::new(self.clients.clone());

        communicator.broadcast(msg)
    }

    fn change_player(&mut self) {
        self.current_player += 1;
        self.current_player %= 2;
    }

    fn send_turn(&self) {
        let msg = api::common::Message::new(api::response::CMD_TURN);
        let communicator = communicator::Communicator::new(self.clients.clone());

        communicator
            .broadcast_everyone_but(&msg, self.clients[self.current_player as usize].clone());
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
        self.next_turn();
        self.game = game;
    }
}
