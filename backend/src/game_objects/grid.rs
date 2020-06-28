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

    pub fn get_hex(&mut self, y: u32, x: u32) -> Option<&mut Hex> {
        self.hexes.iter_mut().find(|hex| hex.x == x && hex.y == y)
    }
}

#[cfg(test)]
mod test {
    use super::super::hex::Hex;
    use super::super::hex_objects::content::Content;
    use super::super::hex_objects::wall::Wall;
    use super::super::unit::Unit;
    use super::Grid;

    #[test]
    fn new() {
        let row_n = 10;
        let col_n = 13;
        let grid = Grid::new(row_n, col_n);

        assert_eq!(grid.hexes.len(), (row_n * col_n) as usize);

        let mut hexes = grid.hexes.into_iter();
        for x in 0..row_n {
            for y in 0..col_n {
                let hex = hexes.next().unwrap();

                assert_eq!(hex.x, x);
                assert_eq!(hex.y, y);
                assert_eq!(hex.unit.is_none(), true);
                assert_eq!(hex.content.is_none(), true);
            }
        }
    }

    #[test]
    fn get_hex() {
        let mut grid = Grid::new(5, 5);

        // hex that exists
        let hex = grid.get_hex(1, 2);

        assert!(hex.is_some());
        let hex = hex.unwrap();
        assert_eq!(hex.y, 1);
        assert_eq!(hex.x, 2);

        // hex that does not exists
        let hex = grid.get_hex(3, 8);
        assert!(hex.is_none());
    }

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
            content: Some(Content::Wall(Wall {})),
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
