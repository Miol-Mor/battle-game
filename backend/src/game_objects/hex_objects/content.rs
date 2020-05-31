use super::wall::Wall;
use serde::Serialize;

#[derive(Clone, Serialize)]
pub enum Content {
    Wall(Wall),
}
