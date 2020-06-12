use crate::game_objects::hex_objects::content::Content;
use crate::game_objects::hex_objects::wall::Wall;
use rstest::*;

#[fixture]
pub fn content_wall() -> Content {
    Content::Wall(Wall {})
}
