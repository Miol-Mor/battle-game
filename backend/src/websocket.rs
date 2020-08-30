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

            self.app_state.process_message(text);
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

                self.app_state.new_game();
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
