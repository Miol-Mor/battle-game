use crate::fixtures;
use crate::game_objects::hex::Hex;
use rstest::*;

#[fixture]
pub fn x() -> u32 {
    1
}

#[fixture]
pub fn y() -> u32 {
    2
}

#[fixture]
pub fn hex_empty() -> Hex {
    Hex {
        x: x(),
        y: y(),
        unit: None,
        content: None,
    }
}

#[fixture]
pub fn hex_with_unit() -> Hex {
    Hex {
        unit: Some(fixtures::unit::unit()),
        ..hex_empty()
    }
}

#[fixture]
pub fn hex_with_wall() -> Hex {
    Hex {
        content: Some(fixtures::content::content_wall()),
        ..hex_empty()
    }
}

#[fixture]
pub fn hex_with_unit_and_wall() -> Hex {
    Hex {
        unit: Some(fixtures::unit::unit()),
        content: Some(fixtures::content::content_wall()),
        ..hex_empty()
    }
}
