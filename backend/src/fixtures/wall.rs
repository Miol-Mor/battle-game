use crate::game_objects::hex_objects::wall::Wall;
use rstest::*;

#[fixture]
pub fn wall() -> Wall {
    Wall {}
}
