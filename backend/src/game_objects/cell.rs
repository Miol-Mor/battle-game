use super::unit::Unit;
use serde::Serialize;

#[derive(Clone, Serialize)]
pub enum CellType {
    Regular,
    Wall,
}

#[derive(Clone, Serialize)]
pub struct Cell {
    pub x: u32,
    pub y: u32,
    pub unit: Option<Unit>,
    pub cell_type: CellType,
}

#[cfg(test)]
mod test {
    use super::{Cell, CellType, Unit};

    #[test]
    fn sertialize_without_unit() {
        let cell = Cell {
            x: 1,
            y: 2,
            unit: None,
            cell_type: CellType::Wall,
        };
        let cell_string = serde_json::to_string(&cell).unwrap();

        assert_eq!(
            cell_string,
            "{\"x\":1,\"y\":2,\"unit\":null,\"cell_type\":\"Wall\"}"
        );
    }

    #[test]
    fn serialize_with_unit() {
        let unit = Unit { hp: 10, attack: 2 };
        let cell = Cell {
            x: 1,
            y: 2,
            unit: Some(unit.clone()),
            cell_type: CellType::Regular,
        };
        let cell_string = serde_json::to_string(&cell).unwrap();
        let unit_string = serde_json::to_string(&unit).unwrap();

        assert_eq!(
            cell_string,
            format!(
                "{{\"x\":1,\"y\":2,\"unit\":{},\"cell_type\":\"Regular\"}}",
                unit_string
            ),
        );
    }
}
