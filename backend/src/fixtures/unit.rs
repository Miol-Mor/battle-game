use crate::game_objects::unit::Unit;
use rstest::*;

#[fixture]
pub fn player() -> u32 {
    1
}

#[fixture]
pub fn hp() -> u32 {
    10
}

#[fixture]
pub fn attack() -> [u32; 2] {
    [2, 3]
}

#[fixture]
pub fn speed() -> u32 {
    3
}

#[fixture]
pub fn unit() -> Unit {
    Unit {
        player: player(),
        hp: hp(),
        attack: attack(),
        speed: speed(),
    }
}
