use crate::config::CONFIG;

use actix::Addr;
use actix_web::middleware::Logger;
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;

use game::Game;

use std::sync::Mutex;

mod api;
mod appstate;
mod auth;
mod config;
mod database;
mod errors;
mod game;
mod game_objects;
mod handlers;
mod helpers;
mod models;
mod routes;
mod websocket;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate log;

async fn index(
    req: HttpRequest,
    stream: web::Payload,
    data: web::Data<appstate::AppState>,
) -> Result<HttpResponse, Error> {
    let res = ws::start(
        websocket::Websocket {
            self_addr: None,
            app_state: data.clone(),
        },
        &req,
        stream,
    );

    let cnt = data.clients.lock().unwrap();
    debug!("Number of clients {}", cnt.len());
    res
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::init();
    let data = web::Data::new(appstate::AppState {
        clients: Mutex::new(vec![]),
    });
    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .route("/ws/", web::get().to(index))
            .wrap(Logger::default())
            .configure(database::add_user_storage)
            .configure(routes::routes)
    })
    .bind(CONFIG.address.clone())?
    .run()
    .await
}
