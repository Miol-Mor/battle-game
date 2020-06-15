use crate::config::CONFIG;

use actix_web::middleware::Logger;
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;

mod auth;
mod config;
mod game;
mod game_objects;
mod handlers;
mod routes;
mod websocket;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate log;

async fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::start(websocket::Websocket {}, &req, stream)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::init();
    HttpServer::new(|| {
        App::new()
            .route("/ws/", web::get().to(index))
            .wrap(Logger::default())
            .configure(routes::routes)
    })
    .bind(CONFIG.address.clone())?
    .run()
    .await
}
