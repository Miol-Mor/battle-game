use super::content::Wall;
use super::unit::Unit;
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct Hex {
    pub x: u32,
    pub y: u32,
    pub unit: Option<Unit>,
    pub content: Option<Wall>,
}

#[cfg(test)]
mod test {
    use super::super::content::{Wall, WallKind};
    use super::super::unit::Unit;
    use super::Hex;

    #[test]
    fn sertialize_without_anything() {
        let cell = Hex {
            x: 1,
            y: 2,
            unit: None,
            content: None,
        };
        let cell_string = serde_json::to_string(&cell).unwrap();

        assert_eq!(
            cell_string,
            "{\"x\":1,\"y\":2,\"unit\":null,\"content\":null}",
        );
    }

    #[test]
    fn serialize_with_unit_and_content() {
        let unit = Unit {
            player: 1,
            hp: 10,
            attack: [2, 4],
            speed: 4,
        };
        let wall = Wall {
            wall: WallKind::Normal,
        };
        let hex = Hex {
            x: 1,
            y: 2,
            unit: Some(unit.clone()),
            content: Some(wall.clone()),
        };
        let hex_string = serde_json::to_string(&hex).unwrap();
        let unit_string = serde_json::to_string(&unit).unwrap();
        let wall_string = serde_json::to_string(&wall).unwrap();

        assert_eq!(
            hex_string,
            format!(
                "{{\"x\":1,\"y\":2,\"unit\":{},\"content\":{}}}",
                unit_string, wall_string,
            ),
        );
    }
}
