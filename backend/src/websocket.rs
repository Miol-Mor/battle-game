use actix::{Actor, StreamHandler};
use actix_web_actors::ws;

use super::game::Game;
use super::game_objects::hex_objects::content::Content;
use super::game_objects::hex_objects::wall::Wall;
use super::game_objects::unit::Unit;

use super::api;

/// Define http actor
pub struct Websocket;

impl Actor for Websocket {
    type Context = ws::WebsocketContext<Self>;
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
                    let response = api::response::Moving::new(vec![moving.from, moving.to]);
                    ctx.text(&serde_json::to_string(&response).unwrap());
                }
                api::request::CMD_ATTACK => {
                    let message = api::request::Attack::from_str(&text);
                    let message = api::response::Attacking::new(message.from, message.to);
                    ctx.text(&serde_json::to_string(&message).unwrap());

                    // After attack turn ends
                    let message = api::common::Message::new(api::response::CMD_TURN);
                    ctx.text(&serde_json::to_string(&message).unwrap());
                }
                _ => {
                    debug!("Unknown command: {}", message.cmd);
                }
            }
        }
    }

    fn started(&mut self, ctx: &mut Self::Context) {
        debug!("Client connected");

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

        ctx.text(&serde_json::to_string(&game).unwrap());
        let message = api::common::Message::new(api::response::CMD_TURN);
        ctx.text(&serde_json::to_string(&message).unwrap())
    }

    fn finished(&mut self, _ctx: &mut Self::Context) {
        debug!("Client disconnected");
    }
}
