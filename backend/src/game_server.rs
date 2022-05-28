// TODO: Refactor it
use actix::{Actor, Addr, Context, Handler};

use serde::Serialize;

use crate::communicator;
use crate::websocket::Websocket;

use crate::api::common::Point;
use crate::api::inner;
use crate::api::request;
use crate::api::request::{Click, SkipTurn, StartGame};
use crate::api::response::{
    Attacking, ConnectionQueue, Deselecting, Die, End, EndState, Error, Field, Hurt, Moving,
    Selecting, State, Update,
};
use crate::game::{Action, Game};
use crate::game_objects::hex_objects::content::Content;
use crate::game_objects::hex_objects::wall::Wall;
use crate::game_objects::unit::Unit;

use eyre::{Result, WrapErr};

// TODO: make it better
const STATE_WAIT: &str = "wait";
const STATE_WATCH: &str = "watch";
const STATE_SELECT: &str = "select";
const STATE_ACTION: &str = "action";
const STATE_ATTACK: &str = "attack";

#[derive(Debug)]
pub struct GameServer {
    pub clients: Vec<Addr<Websocket>>,
    pub game: Game,
    pub current_player: usize,
    pub num_of_players: usize,
    pub game_started: bool,
}

impl Actor for GameServer {
    type Context = Context<Self>;
}

impl Handler<inner::Request<Click>> for GameServer {
    type Result = ();

    fn handle(&mut self, message: inner::Request<Click>, _: &mut Self::Context) -> Self::Result {
        debug!("Handle click");

        let click = message.payload;

        if !self.check_player_turn(&message.sender) {
            // TODO: make error
            debug!("Error: wrong player clicked");
            return;
        }

        // Choose what action should be done now
        if let Err(error) = match self
            .game
            .get_current_action(click.target, self.current_player as u32)
        {
            Ok(action) => {
                debug!("Selected hex: {:?}", self.game.selected_hex);
                debug!("Action: {:?}", action);
                match action {
                    Action::Deselect => Ok(self.deselect_unit()),
                    Action::Select => {
                        // This arm is for selection and reselsection
                        // If we have no unit selected, we can safely call deselect
                        self.deselect_unit();
                        self.select_unit(click.target)
                    }
                    Action::Move => self.move_unit(click.target),
                    Action::Attack => self.attack_unit(click.target),
                }
            }
            Err(error) => Err(error.wrap_err("determinate action")),
        } {
            // If some error occured during choosing action or action itself,
            // we print this error and send it to frontend
            error!("{:?}", error.wrap_err("handle click"));
            self.send_error(request::CMD_CLICK.to_string());
        };
    }
}

impl Handler<inner::Request<SkipTurn>> for GameServer {
    type Result = ();

    fn handle(&mut self, message: inner::Request<SkipTurn>, _: &mut Self::Context) -> Self::Result {
        debug!("Handle skip turn");

        if !self.check_player_turn(&message.sender) {
            // TODO: make error
            debug!("Error: wrong player clicked");
            return;
        }

        self.next_turn()
    }
}

impl Handler<inner::Request<StartGame>> for GameServer {
    type Result = ();

    fn handle(
        &mut self,
        message: inner::Request<StartGame>,
        _: &mut Self::Context,
    ) -> Self::Result {
        debug!("Handle start game");
        if self.game_started {
            debug!("Error: game has been already started");
            return;
        }
        if self.clients.len() < 2 {
            debug!("Error: at least 2 players needed to start game");
            return;
        }
        self.num_of_players = self.clients.len();
        self.new_game();
    }
}

impl Handler<inner::NewClient> for GameServer {
    type Result = ();

    fn handle(&mut self, client: inner::NewClient, _: &mut Self::Context) -> Self::Result {
        self.clients.push(client.address);

        self.broadcast_connection_state();
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

        // Stop game if one of active players leaves
        if self.game_started && index < self.num_of_players {
            self.stop_game();
        }

        self.broadcast_connection_state();
    }
}

impl GameServer {
    pub fn new() -> GameServer {
        GameServer {
            clients: vec![],
            game: Game::new(0, 0),
            current_player: 0,
            num_of_players: 0, // Undefined number of players on the start
            game_started: false,
        }
    }

    // Messages
    pub fn broadcast<T: Serialize>(&self, msg: T) {
        communicator::broadcast(&msg, self.clients.clone())
    }

    fn send_current_player<T: Serialize>(&self, msg: T) {
        // TODO: here and below is potential bug due to self.clients can not have index of self.current_player
        communicator::broadcast(&msg, vec![self.clients[self.current_player].clone()]);
    }

    fn send_other_players<T: Serialize>(&self, msg: T) {
        // TODO: May be optimized
        let mut other_players: Vec<Addr<Websocket>> = Vec::new();
        for i in 0..self.num_of_players {
            if i != self.current_player {
                other_players.push(self.clients[i].clone());
            }
        }
        communicator::broadcast(&msg, other_players);
    }

    fn send_error(&self, error_message: String) {
        let error = Error::new(error_message);
        communicator::broadcast(&error, vec![self.clients[self.current_player].clone()]);
    }

    fn broadcast_connection_state(&self) {
        for (player_number, player_address) in self.clients.iter().enumerate() {
            let msg = &ConnectionQueue::new(
                self.clients.len() as u32,
                (player_number + 1) as u32,
                self.game_started,
            );
            communicator::broadcast(msg, vec![player_address.clone()]);
        }
    }

    // Units
    fn select_unit(&mut self, target: Point) -> Result<()> {
        let selection = self.game.select_unit(target).wrap_err("select unit")?;

        self.send_current_player(&Selecting::new(
            selection.target.to_point(),
            &selection.highlight_hexes,
        ));
        self.send_current_player(&State::new(STATE_ACTION.to_string()));

        Ok(())
    }

    fn deselect_unit(&mut self) {
        if let Some(hex) = self.game.selected_hex {
            self.send_current_player(Deselecting::new(hex.to_point()));
            self.game.deselect_unit();
            self.send_current_player(State::new(STATE_SELECT.to_string()));
        };
    }

    fn move_unit(&mut self, to: Point) -> Result<()> {
        let path = self.game.move_unit(to).wrap_err("move unit")?;
        let hexes = self
            .game
            .hexes_from_points(path)
            .wrap_err("hexes from point")?;
        let message = Moving::new(hexes);

        self.broadcast(&message);

        self.game
            .select_unit(to)
            .wrap_err("select unit after move")?;

        self.send_current_player(State::new(STATE_ATTACK.to_string()));

        Ok(())
    }

    fn attack_unit(&mut self, to: Point) -> Result<()> {
        let (hurt, die) = self.game.attack(to).wrap_err("attack")?;

        // if selected_hex is none, we must fall on .attack function befoure
        let from = self.game.selected_hex.unwrap();

        self.deselect_unit();
        self.broadcast(Attacking::new(from.to_point(), to));
        self.broadcast(Hurt::new(hurt));
        self.broadcast(Die::new(die));
        self.next_turn();

        Ok(())
    }

    // Game logics
    pub fn next_turn(&mut self) {
        let hexes_to_change = self.game.restore_movements(self.current_player as u32);
        let message = Update::new(hexes_to_change);
        debug!("Changes: {:?}", message);
        self.broadcast(message);

        self.deselect_unit();

        if self.game.ends() {
            self.send_current_player(End::new(EndState::Win));
            self.send_other_players(End::new(EndState::Lose));
            self.game_started = false;
            self.broadcast_connection_state();
            return;
        }

        self.send_current_player(State::new(STATE_WAIT.to_string()));
        self.change_player();
        self.send_current_player(State::new(STATE_ACTION.to_string()));

        debug!("Game state: {:?}", self.game);
    }

    fn change_player(&mut self) {
        loop {
            self.current_player += 1;
            self.current_player %= self.num_of_players;
            if self
                .game
                .field
                .players_alive()
                .contains(&(self.current_player as u32))
            {
                break;
            }
        }
    }

    fn check_player_turn(&self, addr: &Addr<Websocket>) -> bool {
        self.clients[self.current_player] == *addr
    }

    pub fn new_game(&mut self) {
        let game = Game::random(self.num_of_players);
        self.broadcast(State::new(STATE_WAIT.to_string()));

        self.broadcast(Field::new(&game));
        // TODO: Make a function broadcast to spectators
        communicator::broadcast(
            &State::new(STATE_WATCH.to_string()),
            self.clients.clone()[self.num_of_players..self.clients.len()].to_vec(),
        );
        self.send_current_player(State::new(STATE_ACTION.to_string()));
        self.game = game;
        self.game_started = true;
    }

    pub fn stop_game(&mut self) {
        debug!("Stop game");
        self.broadcast(End::new(EndState::Disconnected));
        self.game_started = false;
        self.broadcast_connection_state();
    }
}
