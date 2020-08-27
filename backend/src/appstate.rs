use actix::Addr;

use std::sync::Mutex;

use crate::websocket::Websocket;

#[derive(Debug)]
pub struct AppState {
    pub clients: Mutex<Vec<Addr<Websocket>>>,
}
