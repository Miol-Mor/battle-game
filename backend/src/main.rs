use crate::config::CONFIG;

use actix::{Actor, Addr};
use actix_web::middleware::Logger;
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;

mod api;
mod auth;
mod communicator;
mod config;
mod database;
mod errors;
mod game;
mod game_objects;
mod game_server;
mod handlers;
mod helpers;
mod models;
mod routes;
mod websocket;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate log;

use tracing::instrument;

#[instrument(skip(stream))]
async fn index(
    req: HttpRequest,
    stream: web::Payload,
    data: web::Data<Addr<game_server::GameServer>>,
) -> Result<HttpResponse, Error> {
    ws::start(websocket::Websocket { server_addr: data }, &req, stream)
}

#[actix_rt::main]
#[instrument]
async fn main() -> std::io::Result<()> {
    install_tracing();
    color_eyre::install().unwrap();
    let data = web::Data::new(game_server::GameServer::new().start());
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

fn install_tracing() {
    use tracing_error::ErrorLayer;
    use tracing_subscriber::prelude::*;
    use tracing_subscriber::{fmt, EnvFilter};

    let fmt_layer = fmt::layer().with_target(false);
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .with(ErrorLayer::default())
        .init();
}
