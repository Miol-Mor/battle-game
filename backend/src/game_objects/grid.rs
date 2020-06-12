use super::hex::Hex;
use serde::Serialize;

#[derive(Serialize)]
pub struct Grid {
    pub hexes: Vec<Hex>,
}

impl Grid {
    pub fn new(row_n: u32, col_n: u32) -> Grid {
        let hexes: Vec<Hex> = (0..row_n)
            .flat_map(|x| {
                (0..col_n).map(move |y| Hex {
                    x,
                    y,
                    unit: None,
                    content: None,
                })
            })
            .collect();

        Grid { hexes }
    }

    pub fn get_hex(&mut self, x: u32, y: u32) -> Option<&mut Hex> {
        self.hexes.iter_mut().find(|hex| hex.x == x && hex.y == y)
    }
}

#[cfg(test)]
mod test {
    use super::Grid;
    use crate::fixtures;

    #[test]
    fn new() {
        let grid = fixtures::grid::grid();
        let row_n = fixtures::grid::row_n();
        let col_n = fixtures::grid::col_n();

        assert_eq!(grid.hexes.len(), (row_n * col_n) as usize);

        let mut hexes = grid.hexes.into_iter();
        for x in 0..row_n {
            for y in 0..col_n {
                let hex = hexes.next().unwrap();

                assert_eq!(hex.x, x);
                assert_eq!(hex.y, y);
                assert!(hex.unit.is_none());
                assert!(hex.content.is_none());
            }
        }
    }

    #[test]
    fn get_hex() {
        let mut grid = fixtures::grid::grid();

        // hex that exists
        let hex = grid.get_hex(fixtures::grid::x_in_grid(), fixtures::grid::y_in_grid());

        assert!(hex.is_some());
        let hex = hex.unwrap();
        assert_eq!(hex.x, fixtures::grid::x_in_grid());
        assert_eq!(hex.y, fixtures::grid::y_in_grid());

        // hex that does not exists
        let hex = grid.get_hex(fixtures::grid::x_in_grid(), fixtures::grid::y_out_grid());
        assert!(hex.is_none());
    }

    #[test]
    fn serialize() {
        let hex_one = fixtures::hex::hex_empty();
        let hex_two = fixtures::hex::hex_with_unit_and_wall();

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
