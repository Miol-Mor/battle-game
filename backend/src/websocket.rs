use actix::{Actor, Addr, AsyncContext, Handler, Message, StreamHandler};
use actix_web::web;
use actix_web_actors::ws;

use serde::Serialize;

use super::game::Game;
use super::game_objects::hex_objects::content::Content;
use super::game_objects::hex_objects::wall::Wall;
use super::game_objects::unit::Unit;
use crate::appstate::AppState;
use crate::communicator::Msg;

use super::api;

/// Define http actor
#[derive(Debug)]
pub struct Websocket {
    pub self_addr: Option<Addr<Websocket>>,
    pub app_state: web::Data<AppState>,
}

impl Actor for Websocket {
    type Context = ws::WebsocketContext<Self>;
}

// Message received by do_send() method
impl Handler<Msg> for Websocket {
    type Result = ();

    fn handle(&mut self, msg: Msg, ctx: &mut Self::Context) {
        debug!("Sending message {:?}!", msg);
        ctx.text(msg.0);
    }
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for Websocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        if let Ok(ws::Message::Text(text)) = msg {
            debug!("Client text: {}", text);

            let message = api::common::Message::from_str(&text);
            debug!("Client message: {:?}", message);

            debug!(
                "Current player: {:}",
                self.app_state.current_player.lock().unwrap()
            );

            match message.cmd.as_str() {
                api::request::CMD_MOVE => {
                    let moving = api::request::Move::from_str(&text);
                    let mut game = self.app_state.game.lock().unwrap();
                    debug!("{:?}", game);
                    // Just example - how we can change shared data
                    game.set_unit(
                        moving.from.y,
                        moving.from.x,
                        Unit {
                            player: 0,
                            hp: 0,
                            damage: [0, 0],
                            speed: 0,
                        },
                    )
                    .unwrap();
                    let response = api::response::Moving::new(vec![moving.from, moving.to]);
                    self.app_state.broadcast(&response);
                }
                api::request::CMD_ATTACK => {
                    let message = api::request::Attack::from_str(&text);
                    let message = api::response::Attacking::new(message.from, message.to);
                    self.app_state.broadcast(&message);

                    // After attack turn ends
                    self.app_state.next_turn();
                }
                _ => {
                    debug!("Unknown command: {}", message.cmd);
                }
            }
        }
    }

    fn started(&mut self, ctx: &mut Self::Context) {
        let clients_num;
        {
            let mut clients = self.app_state.clients.lock().unwrap();
            clients.push(ctx.address());
            clients_num = clients.len();
        }

        self.self_addr = Some(ctx.address());
        debug!("Client connected");

        match clients_num {
            2 => {
                debug!("Two clients connected, sending game!");
                let mut game = Game::new(2, 2);
                let unit = Unit {
                    player: 1,
                    hp: 1,
                    damage: [1, 2],
                    speed: 1,
                };
                let wall = Wall {};

                match game.set_unit(0, 0, unit) {
                    Ok(_) => {}
                    Err(error) => ctx.text(error),
                }

                match game.set_content(1, 1, Content::Wall(wall)) {
                    Ok(_) => {}
                    Err(error) => ctx.text(error),
                }
                self.app_state.broadcast(&game);
                self.app_state.next_turn();
                *self.app_state.game.lock().unwrap() = game;
            }
            n if n > 2 => {
                ctx.text("{\"cmd\": \"GFY! :D\"}");
            }
            _ => {}
        }
    }

    fn finished(&mut self, _ctx: &mut Self::Context) {
        debug!("Client disconnected");
    }
}
