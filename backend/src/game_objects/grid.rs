use super::hex::Hex;
use serde::Serialize;

#[derive(Serialize)]
pub struct Grid {
    pub hexes: Vec<Hex>,
}

#[cfg(test)]
mod test {
    use super::super::hex::Hex;
    use super::super::hex_objects::content::Content;
    use super::super::hex_objects::wall::{Wall, WallKind};
    use super::super::unit::Unit;
    use super::Grid;

    #[test]
    fn serialize() {
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
            content: Some(Content::Wall(Wall {
                kind: WallKind::Default,
            })),
        };
        let grid = Grid {
            hexes: vec![hex_one.clone(), hex_two.clone()],
        };
        let grid_string = serde_json::to_string(&grid).unwrap();
        let hex_one_string = serde_json::to_string(&hex_one).unwrap();
        let hex_two_string = serde_json::to_string(&hex_two).unwrap();

        assert_eq!(
            grid_string,
            format!("{{\"hexes\":[{},{}]}}", hex_one_string, hex_two_string),
        );
    }
}
