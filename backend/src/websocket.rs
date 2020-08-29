use actix::{Actor, Addr, AsyncContext, Handler, Message, StreamHandler};
use actix_web::web;
use actix_web_actors::ws;

use serde::Serialize;

use super::game::Game;
use super::game_objects::hex_objects::content::Content;
use super::game_objects::hex_objects::wall::Wall;
use super::game_objects::unit::Unit;
use crate::appstate::AppState;

use super::api;

/// Define http actor
#[derive(Debug)]
pub struct Websocket {
    pub self_addr: Option<Addr<Websocket>>,
    pub app_state: web::Data<AppState>,
}

#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct Msg(pub String);

impl Websocket {
    fn broadcast<T: Serialize>(&self, msg: &T) {
        let clients = self.app_state.clients.lock().unwrap();
        let m = serde_json::to_string(&msg).unwrap();
        debug!("Sending {} to all clients", m);
        for c in &*clients {
            c.do_send(Msg(m.clone()));
        }
    }

    fn send_turn(&self) {
        let message = api::common::Message::new(api::response::CMD_TURN);
        let clients = self.app_state.clients.lock().unwrap();
        debug!("Sending turn to other player");
        for c in &*clients {
            if *c != *self.self_addr.as_ref().unwrap() {
                c.do_send(Msg(serde_json::to_string(&message).unwrap()));
                break;
            }
        }
    }
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
                            attack: [0, 0],
                            speed: 0,
                        },
                    )
                    .unwrap();
                    let response = api::response::Moving::new(vec![moving.from, moving.to]);
                    self.broadcast(&response);
                }
                api::request::CMD_ATTACK => {
                    let message = api::request::Attack::from_str(&text);
                    let message = api::response::Attacking::new(message.from, message.to);
                    self.broadcast(&message);

                    // After attack turn ends
                    self.send_turn();
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
                    attack: [1, 2],
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
                self.broadcast(&game);
                self.send_turn();
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
