use super::cell::Cell;
use serde::Serialize;

#[derive(Serialize)]
pub struct Grid {
    pub cells: Vec<Cell>,
}

#[cfg(test)]
mod test {
    use super::super::cell::{Cell, CellType};
    use super::super::unit::Unit;
    use super::Grid;

    #[test]
    fn sertialize() {
        let unit = Unit { hp: 1, attack: 120 };
        let cell_one = Cell {
            x: 1,
            y: 1,
            unit: None,
            cell_type: CellType::Regular,
        };
        let cell_two = Cell {
            x: 1,
            y: 2,
            unit: Some(unit),
            cell_type: CellType::Regular,
        };
        let grid = Grid {
            cells: vec![cell_one.clone(), cell_two.clone()],
        };
        let grid_string = serde_json::to_string(&grid).unwrap();
        let cell_one_string = serde_json::to_string(&cell_one).unwrap();
        let cell_two_string = serde_json::to_string(&cell_two).unwrap();

        assert_eq!(
            grid_string,
            format!("{{\"cells\":[{},{}]}}", cell_one_string, cell_two_string),
        );
    }
}
