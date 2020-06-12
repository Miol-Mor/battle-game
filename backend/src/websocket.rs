use actix::{Actor, StreamHandler};
use actix_web_actors::ws;

use super::game::Game;
use super::game_objects::hex_objects::content::Content;
use super::game_objects::hex_objects::wall::Wall;
use super::game_objects::unit::Unit;

/// Define http actor
pub struct Websocket;

impl Actor for Websocket {
    type Context = ws::WebsocketContext<Self>;
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for Websocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => {
                debug!("Client text: {}", text);
                ctx.text(&text)
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
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

        // TODO: process error on unwrap
        ctx.text(&serde_json::to_string(&game).unwrap());
    }

    fn finished(&mut self, _ctx: &mut Self::Context) {
        debug!("Client disconnected");
    }
}
