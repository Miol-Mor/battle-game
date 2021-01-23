use actix::{Actor, Addr, Context, Handler};

use serde::Serialize;

use crate::communicator;
use crate::websocket::Websocket;

use crate::api::common::Point;
use crate::api::inner;
use crate::api::request;
use crate::api::request::{Click, SkipTurn};
use crate::api::response::{
    Attacking, Deselecting, Die, Error, Field, Hurt, Moving, Selecting, State, Update,
};
use crate::game::Game;
use crate::game_objects::hex_objects::content::Content;
use crate::game_objects::hex_objects::wall::Wall;
use crate::game_objects::unit::Unit;

// TODO: make it better
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
            None => {
                self.select_unit(click.target);
                return;
            }
            Some(hex) => hex,
        };

        if hex.to_point() == click.target {
            self.deselect_unit();
        } else {
            self.unit_action(hex.unit.clone(), hex.to_point(), click.target);
        }
    }
}

impl Handler<inner::Request<SkipTurn>> for GameServer {
    type Result = ();

    fn handle(&mut self, message: inner::Request<SkipTurn>, _: &mut Self::Context) -> Self::Result {
        debug!("Handle skip turn");

        if !self.check_player_turn(&message.sender) {
            debug!("Error: wrong player clicked");
            return;
        }

        self.next_turn()
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
                            self.deselect_unit();
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
                if self.current_player as u32 != selection.target.get_unit().unwrap().player {
                    self.game.selected_hex = None;
                    return;
                }
                self.send_current_player(&Selecting::new(
                    selection.target.to_point(),
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
                let hexes = match self.game.hexes_from_points(path) {
                    Ok(hexes) => hexes,
                    Err(err) => {
                        error!("{:?}", err.wrap_err("hexes from point"));
                        self.send_error(request::CMD_CLICK.to_string());
                        return;
                    }
                };
                let message = Moving::new(hexes);
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
                self.broadcast(Attacking::new(from, to));
                self.broadcast(Hurt::new(hurt));
                self.broadcast(Die::new(die));
                self.next_turn();
            }
            Err(error) => {
                error!("{:?}", error.wrap_err("attack handle error"));
                self.send_error(request::CMD_CLICK.to_string());
            }
        }
    }

    pub fn next_turn(&mut self) {
        let hexes_to_change = self.game.restore_movements(self.current_player as u32);
        let message = Update::new(hexes_to_change);
        debug!("Changes: {:?}", message);
        self.broadcast(message);

        self.deselect_unit();

        self.send_current_player(State::new(STATE_WAIT.to_string()));
        self.change_player();
        self.send_current_player(State::new(STATE_ACTION.to_string()));
        debug!("Game state: {:?}", self.game);
    }

    pub fn broadcast<T: Serialize>(&self, msg: T) {
        communicator::broadcast(&msg, self.clients.clone())
    }

    fn change_player(&mut self) {
        self.current_player += 1;
        self.current_player %= 2;
    }

    fn send_current_player<T: Serialize>(&self, msg: T) {
        communicator::broadcast(&msg, vec![self.clients[self.current_player].clone()]);
    }

    fn send_error(&self, error_message: String) {
        let error = Error::new(error_message);
        communicator::broadcast(&error, vec![self.clients[self.current_player].clone()]);
    }

    pub fn new_game(&mut self) {
        let num_x = 4;
        let num_y = 3;

        let mut game = Game::new(num_x, num_y);
        self.broadcast(State::new(STATE_WAIT.to_string()));

        match game.set_unit(0, 0, Some(Unit::new(0, 9, [2, 5], 1))) {
            Ok(_) => {}
            Err(error) => debug!("{:?}", error),
        }
        match game.set_unit(0, 2, Some(Unit::new(0, 3, [1, 2], 2))) {
            Ok(_) => {}
            Err(error) => debug!("{:?}", error),
        }
        match game.set_unit(2, 0, Some(Unit::new(1, 5, [0, 3], 1))) {
            Ok(_) => {}
            Err(error) => debug!("{:?}", error),
        }
        match game.set_unit(3, 2, Some(Unit::new(1, 7, [4, 6], 2))) {
            Ok(_) => {}
            Err(error) => debug!("{:?}", error),
        }

        match game.set_content(1, 1, Some(Content::Wall(Wall {}))) {
            Ok(_) => {}
            Err(error) => debug!("{:?}", error),
        }
        match game.set_content(2, 2, Some(Content::Wall(Wall {}))) {
            Ok(_) => {}
            Err(error) => debug!("{:?}", error),
        }

        self.broadcast(Field::new(&game, num_x, num_y));
        self.send_current_player(State::new(STATE_ACTION.to_string()));
        self.game = game;
    }
}
