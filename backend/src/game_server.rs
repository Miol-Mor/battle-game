use actix::{Actor, Addr, Context, Handler};

use serde::Serialize;

use crate::communicator;
use crate::websocket::Websocket;

use crate::api::common::Point;
use crate::api::inner;
use crate::api::request;
use crate::api::request::Click;
use crate::api::response;
use crate::api::response::{Attacking, Deselecting, Field, Moving, Selecting, State};
use crate::game::Game;
use crate::game_objects::hex_objects::content::Content;
use crate::game_objects::hex_objects::wall::Wall;
use crate::game_objects::unit::Unit;

const STATE_WAIT: &str = "wait";
const STATE_SELECT: &str = "select";
const STATE_ACTION: &str = "action";
const STATE_ATTACK: &str = "attack";

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

impl Handler<inner::Request<Click>> for GameServer {
    type Result = ();

    fn handle(&mut self, message: inner::Request<Click>, _: &mut Self::Context) -> Self::Result {
        debug!("Handle click");
        if !self.check_player_turn(&message.sender) {
            debug!("Error: wrong player clicked");
            return;
        }
        let click = message.payload;

        // https://github.com/rust-lang/rust/issues/59159#issuecomment-539997185
        let hex = match &self.game.selected_hex {
            None => return self.select_unit(click.target),
            Some(hex) => hex,
        };

        if hex.to_point() == click.target {
            self.deselect_unit();
        } else {
            self.unit_action(hex.unit.clone(), hex.to_point(), click.target);
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

impl Handler<inner::LooseClient> for GameServer {
    type Result = ();

    fn handle(&mut self, client: inner::LooseClient, _: &mut Self::Context) -> Self::Result {
        let index = self
            .clients
            .iter()
            .position(|address| *address == client.address)
            .unwrap();

        self.clients.remove(index);
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

    fn unit_action(&mut self, selected_unit: Option<Unit>, from: Point, target: Point) {
        match self.game.get_unit(target.x, target.y) {
            Err(error) => {
                error!("{:?}", error.wrap_err("unit action error"));
                self.send_error(request::CMD_CLICK.to_string());
            }
            Ok(None) => {
                self.move_unit(from, target);
            }
            Ok(Some(target_unit)) => {
                match selected_unit {
                    None => {
                        error!("No unit found in hex {:?}", from);
                        self.send_error(request::CMD_CLICK.to_string());
                    }
                    Some(selected_unit) => {
                        if target_unit.player == selected_unit.player {
                            // just select another unit
                            self.select_unit(target);
                        } else {
                            // enemy unit: attack!
                            self.attack_unit(from, target);
                        }
                    }
                }
            }
        }
    }

    fn select_unit(&mut self, target: Point) {
        match self.game.select_unit(target) {
            Ok(selection) => {
                self.send_current_player(&Selecting::new(
                    selection.target,
                    &selection.highlight_hexes,
                ));
                self.send_current_player(&State::new(STATE_ACTION.to_string()));
            }
            Err(error) => {
                error!("{:?}", error.wrap_err("select handle error"));
                self.send_error(request::CMD_CLICK.to_string());
            }
        }
    }

    fn deselect_unit(&mut self) {
        match &self.game.selected_hex {
            None => {}
            Some(hex) => {
                self.send_current_player(Deselecting::new(hex.to_point()));
                self.game.deselect_unit();
                self.send_current_player(State::new(STATE_SELECT.to_string()));
            }
        }
    }

    fn move_unit(&mut self, from: Point, to: Point) {
        match self.game.move_unit(from, to) {
            Ok(path) => {
                let message = Moving::new(path);
                self.broadcast(&message);
                match self.game.select_unit(to) {
                    Ok(_) => {}
                    Err(error) => {
                        error!("{:?}", error.wrap_err("select after move"));
                        self.send_error(request::CMD_CLICK.to_string());
                    }
                }
                self.send_current_player(State::new(STATE_ATTACK.to_string()))
            }
            Err(error) => {
                error!("{:?}", error.wrap_err("move handle error"));
                self.send_error(request::CMD_CLICK.to_string());
            }
        }
    }

    fn attack_unit(&mut self, from: Point, to: Point) {
        match self.game.attack(from, to) {
            Ok((hurt, die)) => {
                self.deselect_unit();
                let message = Attacking::new(from, to, hurt, die);
                self.broadcast(&message);
                self.next_turn();
            }
            Err(error) => {
                error!("{:?}", error.wrap_err("attack handle error"));
                self.send_error(request::CMD_CLICK.to_string());
            }
        }
    }

    pub fn next_turn(&mut self) {
        self.send_current_player(State::new(STATE_WAIT.to_string()));
        self.change_player();
        self.send_current_player(State::new(STATE_ACTION.to_string()));
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
        self.broadcast(&State::new(STATE_WAIT.to_string()));

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
