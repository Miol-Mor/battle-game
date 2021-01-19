use actix::{Actor, Addr, AsyncContext, Handler, StreamHandler};
use actix_web::web;
use actix_web_actors::ws;

use crate::communicator::Msg;
use crate::game_server::GameServer;

use super::api;

/// Define http actor
#[derive(Debug)]
pub struct Websocket {
    pub server_addr: web::Data<Addr<GameServer>>,
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
            match message.cmd.as_str() {
                api::request::CMD_CLICK => {
                    let message = api::request::Click::from_str(&text);
                    let inner_message = api::inner::Request::new(ctx.address(), message);
                    self.server_addr.do_send(inner_message);
                }
                api::request::CMD_SKIP_TURN => {
                    let message = api::request::SkipTurn {};
                    let inner_message = api::inner::Request::new(ctx.address(), message);
                    self.server_addr.do_send(inner_message);
                }
                _ => {
                    debug!("Unknown command: {}", message.cmd);
                }
            }
        }
    }

    fn started(&mut self, ctx: &mut Self::Context) {
        debug!("Client connected");
        self.server_addr
            .do_send(api::inner::NewClient::new(ctx.address()));
    }

    fn finished(&mut self, ctx: &mut Self::Context) {
        debug!("Client disconnected");
        self.server_addr
            .do_send(api::inner::LooseClient::new(ctx.address()));
    }
}
