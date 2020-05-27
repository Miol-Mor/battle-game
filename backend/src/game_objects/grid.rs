use super::hex::Hex;
use serde::Serialize;

#[derive(Serialize)]
pub struct Grid {
    pub hexs: Vec<Hex>,
}

#[cfg(test)]
mod test {
    use super::super::content::{Wall, WallKind};
    use super::super::hex::Hex;
    use super::super::unit::Unit;
    use super::Grid;

    #[test]
    fn sertialize() {
        let unit = Unit {
            player: 1,
            hp: 1,
            attack: [120, 130],
            speed: 1,
        };
        let hex_one = Hex {
            x: 1,
            y: 1,
            unit: None,
            content: None,
        };
        let hex_two = Hex {
            x: 1,
            y: 2,
            unit: Some(unit),
            content: Some(Wall {
                wall: WallKind::Normal,
            }),
        };
        let grid = Grid {
            hexs: vec![hex_one.clone(), hex_two.clone()],
        };
        let grid_string = serde_json::to_string(&grid).unwrap();
        let hex_one_string = serde_json::to_string(&hex_one).unwrap();
        let hex_two_string = serde_json::to_string(&hex_two).unwrap();

        assert_eq!(
            grid_string,
            format!("{{\"hexs\":[{},{}]}}", hex_one_string, hex_two_string),
        );
    }
}
