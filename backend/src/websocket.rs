use actix::{Actor, Addr, AsyncContext, Handler, StreamHandler};
use actix_web::web;
use actix_web_actors::ws;

use crate::appstate::AppState;
use crate::communicator::Msg;

use super::api;

/// Define http actor
#[derive(Debug)]
pub struct Websocket {
    pub app_state_addr: web::Data<Addr<AppState>>,
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
                api::request::CMD_MOVE => {
                    let message = api::request::Move::from_str(&text);
                    let inner_message = api::inner::Request::new(ctx.address(), message);

                    self.app_state_addr.do_send(inner_message);
                }
                api::request::CMD_ATTACK => {
                    let message = api::request::Attack::from_str(&text);
                    let inner_message = api::inner::Request::new(ctx.address(), message);

                    self.app_state_addr.do_send(inner_message);
                }
                _ => {
                    debug!("Unknown command: {}", message.cmd);
                }
            }
        }
    }

    fn started(&mut self, ctx: &mut Self::Context) {
        debug!("Client connected");
        self.app_state_addr
            .do_send(api::request::NewClient::new(ctx.address()));
    }

    fn finished(&mut self, _ctx: &mut Self::Context) {
        debug!("Client disconnected");
    }
}
