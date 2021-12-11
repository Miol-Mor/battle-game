use super::hex::Hex;
use serde::Serialize;
use std::collections::HashSet;

#[derive(Serialize, Debug, Clone)]
pub struct Grid {
    pub num_x: u32,
    pub num_y: u32,
    pub hexes: Vec<Hex>,
}

impl Grid {
    pub fn new(num_x: u32, num_y: u32) -> Grid {
        let hexes: Vec<Hex> = (0..num_x)
            .flat_map(|x| {
                (0..num_y).map(move |y| Hex {
                    x,
                    y,
                    unit: None,
                    content: None,
                })
            })
            .collect();

        Grid {
            num_x,
            num_y,
            hexes,
        }
    }

    // get hex with further change
    pub fn get_hex_mut(&mut self, x: u32, y: u32) -> Option<&mut Hex> {
        self.hexes.iter_mut().find(|hex| hex.x == x && hex.y == y)
    }

    // get hex with further change
    pub fn get_hex(&self, x: u32, y: u32) -> Option<Hex> {
        match self.hexes.iter().find(|hex| hex.x == x && hex.y == y) {
            None => None,
            Some(hex) => Some(*hex),
        }
    }

    // get number of players on the field
    pub fn players_alive(&self) -> HashSet<u32> {
        let mut players = HashSet::new();

        for hex in &self.hexes {
            if let Some(unit) = hex.unit {
                players.insert(unit.player);
            };
        }

        players
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
        let num_x = 13;
        let num_y = 10;
        let grid = Grid::new(num_x, num_y);

        assert_eq!(grid.hexes.len(), (num_x * num_y) as usize);

        let mut hexes = grid.hexes.into_iter();
        for x in 0..num_x {
            for y in 0..num_y {
                let hex = hexes.next().unwrap();

                assert_eq!(hex.x, x);
                assert_eq!(hex.y, y);
                assert_eq!(hex.unit.is_none(), true);
                assert_eq!(hex.content.is_none(), true);
            }
        }
    }

    #[test]
    fn get_hex_mut() {
        let mut grid = Grid::new(5, 5);

        // hex that exists
        let hex = grid.get_hex_mut(1, 2);

        assert!(hex.is_some());
        let hex = hex.unwrap();
        assert_eq!(hex.x, 1);
        assert_eq!(hex.y, 2);

        // hex that does not exists
        let hex = grid.get_hex_mut(3, 8);
        assert!(hex.is_none());
    }

    #[test]
    fn serialize() {
        let unit = Unit::new(1, 1, [120, 130], 1);
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
            num_x: 1,
            num_y: 2,
            hexes: vec![hex_one, hex_two],
        };
        let grid_string = serde_json::to_string(&grid).unwrap();
        let hex_one_string = serde_json::to_string(&hex_one).unwrap();
        let hex_two_string = serde_json::to_string(&hex_two).unwrap();

        assert_eq!(
            grid_string,
            format!(
                "{{\"num_x\":1,\"num_y\":2,\"hexes\":[{},{}]}}",
                hex_one_string, hex_two_string
            ),
        );
    }
}
