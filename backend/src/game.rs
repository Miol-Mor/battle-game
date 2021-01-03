use crate::api::common::Point;
use crate::game_objects::grid::Grid;
use crate::game_objects::hex::Hex;
use crate::game_objects::hex_objects::content::Content;
use crate::game_objects::unit::Unit;

use eyre::{Result, WrapErr};
use rand::Rng;
use std::collections::HashMap;
use thiserror::Error;
use tracing::instrument;

#[derive(Debug, Clone)]
pub struct Game {
    pub field: Grid,
    pub selected_hex: Option<Hex>,
}

#[derive(Error, Debug, PartialEq)]
pub enum GameError {
    #[error("no hex")]
    NoHex,

    #[error("no unit")]
    NoUnit,

    #[error("no moves")]
    NoMoves,

    #[error("wrong hex")]
    WrongHex,
}

#[derive(Debug)]
pub struct Selection {
    pub target: Hex,
    pub highlight_hexes: Vec<Point>,
}

impl Game {
    // Public api
    pub fn new(num_x: u32, num_y: u32) -> Game {
        Game {
            field: Grid::new(num_x, num_y),
            selected_hex: None,
        }
    }

    #[instrument(skip(self))]
    // return target point and vector of points to highlight
    pub fn select_unit(&mut self, target: Point) -> Result<Selection> {
        match self.get_unit(target.x, target.y).wrap_err("select unit")? {
            Some(_) => {
                self.selected_hex = self.get_hex(target.x, target.y);
                Ok(Selection {
                    target: self.selected_hex.clone().unwrap(),
                    highlight_hexes: self.available_points(&self.selected_hex),
                })
            }
            None => Err(GameError::NoUnit).wrap_err("select unit")?,
        }
    }

    #[instrument(skip(self))]
    // return target and vector of hexes to highlight
    pub fn deselect_unit(&mut self) {
        self.selected_hex = None
    }

    #[instrument(skip(self))]
    pub fn move_unit(&mut self, from: Point, to: Point) -> Result<Vec<Point>> {
        let from_hex = match self.get_hex(from.x, from.y) {
            Some(hex) => hex,
            None => Err(GameError::NoHex).wrap_err("no hex to move unit from")?,
        };

        let unit = match from_hex.get_unit() {
            Some(unit) => unit,
            None => Err(GameError::NoUnit).wrap_err("no unit on move")?,
        };

        if unit.movements == 0 {
            return Err(GameError::NoMoves).wrap_err("no moves left")?;
        }

        let mut hexmap: HashMap<Point, u32> = HashMap::with_capacity(self.field.hexes.len());

        self.fill_path_hexmap(&from_hex, 0, unit.movements, &mut hexmap);

        let path = match self.restore_path_from_hexmap(from, to, &hexmap) {
            Ok(path) => path,
            Err(e) => return Err(e.wrap_err("restore path")),
        };

        // TODO: try to remove double get_hex(from.x, from.y)
        let from_hex = match self.get_hex_mut(from.x, from.y) {
            Some(hex) => hex,
            None => Err(GameError::NoHex).wrap_err("hex from disappeared after pathfinding")?,
        };

        let mut unit = match from_hex.get_unit() {
            Some(unit) => unit,
            None => Err(GameError::NoUnit).wrap_err("unit disappeared after pathfinding")?,
        };

        from_hex.set_unit(None);

        let to_hex = match self.get_hex_mut(to.x, to.y) {
            Some(hex) => hex,
            None => Err(GameError::NoHex).wrap_err("hex to disappeared after pathfinding")?,
        };

        unit.change_movements(unit.movements);
        to_hex.set_unit(Some(unit));

        Ok(path)
    }

    #[instrument(skip(self))]
    pub fn attack(&mut self, from: Point, to: Point) -> Result<(Vec<Hex>, Vec<Hex>)> {
        let from_hex = match self.get_hex(from.x, from.y) {
            Some(hex) => hex,
            None => Err(GameError::NoHex).wrap_err("attack from")?,
        };

        let from_unit = match from_hex.get_unit() {
            Some(unit) => unit,
            None => Err(GameError::NoUnit).wrap_err("attack from")?,
        };

        // Check if we can attack to target hex
        // Due to borrow rules we have to get unmuted hex here
        match self.get_hex(to.x, to.y) {
            Some(to_hex) => {
                if !self.find_neighbours(&from_hex.to_point()).contains(&to_hex) {
                    Err(GameError::WrongHex).wrap_err("not neighbour")?;
                }
            }
            None => Err(GameError::NoHex).wrap_err("attack to")?,
        };

        let to_hex = match self.get_hex_mut(to.x, to.y) {
            Some(hex) => hex,
            None => Err(GameError::NoHex).wrap_err("attack to")?,
        };

        let to_unit = match to_hex.get_unit_mut() {
            Some(unit) => unit,
            None => Err(GameError::NoUnit).wrap_err("attack to")?,
        };

        let mut rng = rand::thread_rng();
        let dmg = rng.gen_range(from_unit.damage[0], from_unit.damage[1] + 1);
        to_unit.change_hp(-(dmg as i32));

        let mut hurt: Vec<Hex> = vec![];
        let mut die: Vec<Hex> = vec![];
        if to_unit.hp == 0 {
            die.push(*to_hex);
            to_hex.set_unit(None);
        } else {
            hurt.push(*to_hex);
        }
        Ok((hurt, die))
    }

    // Private api

    // Unit staff
    // TODO: remove pub after creating new games from presets
    #[instrument(skip(self))]
    pub fn set_unit(&mut self, x: u32, y: u32, unit: Option<Unit>) -> Result<()> {
        match self.get_hex_mut(x, y) {
            Some(hex) => {
                hex.set_unit(unit);
                Ok(())
            }
            None => Err(GameError::NoHex).wrap_err("set unit")?,
        }
    }

    #[instrument(skip(self))]
    pub fn get_unit(&mut self, x: u32, y: u32) -> Result<Option<Unit>> {
        match self.get_hex_mut(x, y) {
            Some(hex) => Ok(hex.get_unit()),
            None => Err(GameError::NoHex).wrap_err("get unit")?,
        }
    }

    #[instrument(skip(self))]
    pub fn get_unit_mut(&mut self, x: u32, y: u32) -> Result<Option<&mut Unit>> {
        match self.get_hex_mut(x, y) {
            Some(hex) => Ok(hex.get_unit_mut()),
            None => Err(GameError::NoHex).wrap_err("get unit")?,
        }
    }

    fn get_unit_hexes_for_player_mut(&mut self, player: u32) -> Vec<&mut Hex> {
        self.field
            .hexes
            .iter_mut()
            .filter(|hex| hex.get_unit().is_some())
            .filter(|hex| hex.get_unit().unwrap().player == player)
            .collect()
    }

    pub fn restore_movements(&mut self, player: u32) -> Vec<Hex> {
        let mut hexes = vec![];
        for hex in self.get_unit_hexes_for_player_mut(player) {
            let unit = hex.get_unit_mut().unwrap();
            if unit.has_moved() {
                unit.restore_movements();
                hexes.push(hex.clone());
            }
        }
        hexes
    }

    // Hex stuff
    #[instrument(skip(self))]
    fn get_hex_mut(&mut self, x: u32, y: u32) -> Option<&mut Hex> {
        self.field.get_hex_mut(x, y)
    }

    #[instrument(skip(self))]
    fn get_hex(&self, x: u32, y: u32) -> Option<Hex> {
        self.field.get_hex(x, y)
    }

    #[instrument(skip(self))]
    pub fn hexes_from_points(&self, points: Vec<Point>) -> Result<Vec<Hex>> {
        let mut result = Vec::with_capacity(points.len());

        for point in points {
            match self.get_hex(point.x, point.y) {
                Some(hex) => result.push(hex),
                None => Err(GameError::NoHex)
                    .wrap_err_with(|| format!("no hex in point {:?}", point))?,
            }
        }

        Ok(result)
    }

    // Content stuff
    // TODO: remove pub after creating new games from presets
    // Set content to the point (x, y)
    #[instrument(skip(self))]
    pub fn set_content(&mut self, x: u32, y: u32, content: Option<Content>) -> Result<()> {
        match self.get_hex_mut(x, y) {
            Some(hex) => {
                hex.set_content(content);
                Ok(())
            }
            None => Err(GameError::NoHex).wrap_err("set content")?,
        }
    }

    // Return vector of points wich are reachable by unit in given hex
    // If given hex is non vector is empty
    // If there is no unit in hex, vector is empty
    pub fn available_points(&self, from: &Option<Hex>) -> Vec<Point> {
        let from_hex = match from {
            Some(hex) => hex,
            None => return vec![],
        };

        let unit = match from_hex.unit {
            Some(unit) => unit,
            None => return vec![],
        };

        let mut hexmap: HashMap<Point, u32> = HashMap::with_capacity(self.field.hexes.len());

        self.fill_path_hexmap(from_hex, 0, unit.movements, &mut hexmap);
        hexmap.into_keys().collect()
    }

    // TODO: We don't need two different functions for find available points and find path
    fn fill_path_hexmap(
        &self,
        hex: &Hex,
        value: u32,
        max_value: u32,
        hexmap: &mut HashMap<Point, u32>,
    ) {
        if value > max_value {
            return;
        }
        let hexmap_value = hexmap.get(&hex.to_point());
        // We proceed if we were not in this hex
        // or if we were here, but through longer path
        if hexmap_value.is_none() || hexmap_value.unwrap() > &value {
            if hex.get_unit().is_some() && value != 0 {
                return;
            }
            if hex.get_content().is_some() {
                return;
            }

            hexmap.insert(hex.to_point(), value);
            for hex in self.find_neighbours(&hex.to_point()) {
                self.fill_path_hexmap(&hex, value + 1, max_value, hexmap)
            }
        }
    }

    fn find_neighbours(&self, point: &Point) -> Vec<Hex> {
        let mut hexes: Vec<Hex> = Vec::with_capacity(6);
        let min_x: u32 = if point.x == 0 { point.x } else { point.x - 1 };
        let min_y: u32 = if point.y == 0 { point.y } else { point.y - 1 };
        for x in min_x..point.x + 2 {
            for y in min_y..point.y + 2 {
                if x == point.x && y == point.y {
                    continue;
                }
                let new_hex = match self.get_hex(x, y) {
                    Some(hex) => hex,
                    None => continue,
                };

                // We iterate through nine hexes, but hex has only six neighbours
                // For even and odd rows we need to drop different hexes,
                // that are defined by this formulas
                if point.y % 2 == 0 {
                    if x == point.x.overflowing_sub(1).0 && y != point.y {
                        continue;
                    }
                } else if x == point.x + 1 && y != point.y {
                    continue;
                }

                hexes.push(new_hex);
            }
        }

        hexes
    }

    fn restore_path_from_hexmap(
        &self,
        from: Point,
        to: Point,
        hexmap: &HashMap<Point, u32>,
    ) -> Result<Vec<Point>> {
        if !hexmap.contains_key(&from) {
            Err(GameError::NoHex).wrap_err("no start hex in hexmap")?
        }

        let path_length = match hexmap.get(&to) {
            Some(length) => *length + 1,
            None => Err(GameError::NoHex).wrap_err("no finish hex in hexmap")?,
        };

        let mut path = Vec::with_capacity(path_length as usize);
        path.push(to);

        for i in (0..path_length - 1).rev() {
            // filter all hexes which are neighbours of previous hex in path
            // then find at least one neighbour with needed distance from initial point
            match hexmap
                .iter()
                .filter(|&(k, _)| {
                    self.find_neighbours(
                        &path.last().expect("empty path in restore path algorithm"),
                    )
                    .iter()
                    .map(|hex| hex.to_point())
                    .any(|point| point == *k)
                })
                .find(|&(_, v)| *v == i)
            {
                Some((point, _)) => path.push(*point),
                None => Err(GameError::NoHex)
                    .wrap_err_with(|| format!("no hex in hexmap for distance {}", i))?,
            }
        }

        // path was restored from last point to first, so we reverse it
        path.reverse();
        Ok(path)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::game_objects::hex_objects::wall::Wall;

    // Test game is a game with 2x2 field
    // There are 2 units (U) and 1 wall (W) on it
    // U | W
    // -----
    //   | U
    fn test_game() -> (Game, Unit, Wall) {
        let mut game = Game::new(2, 2);
        let unit = Unit::new(1, 5, [5, 5], 3);
        let wall = Wall {};
        assert!(game.set_unit(0, 0, Some(unit.clone())).is_ok());
        assert!(game.set_unit(1, 1, Some(unit.clone())).is_ok());
        assert!(game.set_content(1, 0, Some(Content::Wall(wall))).is_ok());

        (game, unit, wall)
    }

    #[test]
    fn get_hex_mut() {
        let mut game = Game::new(2, 2);

        let hex = game.get_hex_mut(0, 1);
        assert!(hex.is_some());

        let hex = hex.unwrap();
        assert_eq!(hex.x, 0);
        assert_eq!(hex.y, 1);

        let hex = game.get_hex_mut(5, 6);
        assert!(hex.is_none());
    }

    #[test]
    fn set_get_unit() {
        let (mut game, unit, _) = test_game();

        assert!(game.get_unit(0, 0).unwrap().is_some());
        assert!(game.get_unit(1, 0).unwrap().is_none());
        assert!(game.get_unit(0, 1).unwrap().is_none());

        let result = game.set_unit(4, 5, Some(unit));
        assert!(result.is_err());
        match result.unwrap_err().downcast_ref::<GameError>() {
            Some(GameError::NoHex) => {}
            _ => unreachable!("wrong error"),
        }

        let result = game.get_unit(4, 5);
        assert!(result.is_err());
        match result.unwrap_err().downcast_ref::<GameError>() {
            Some(GameError::NoHex) => {}
            _ => unreachable!("wrong error"),
        }
    }

    #[test]
    fn set_content() {
        let (mut game, _, wall) = test_game();

        let result = game.set_content(4, 5, Some(Content::Wall(wall)));
        assert!(result.is_err());
        match result.unwrap_err().downcast_ref::<GameError>() {
            Some(GameError::NoHex) => {}
            _ => unreachable!("wrong error"),
        }
    }

    #[test]
    fn move_unit_success() {
        let (mut game, _, _) = test_game();
        let from = Point { x: 0, y: 0 };
        let to = Point { x: 0, y: 1 };
        assert!(game.move_unit(from, to).is_ok());
        assert!(game.get_unit(from.x, from.y).unwrap().is_none());
        assert!(game.get_unit(to.x, to.y).unwrap().is_some());
    }

    #[test]
    fn move_unit_into_unit() {
        let (mut game, _, _) = test_game();
        let from = Point { x: 0, y: 0 };
        let to = Point { x: 1, y: 1 };
        assert!(game.move_unit(from, to).is_err());
        assert!(game.get_unit(from.x, from.y).unwrap().is_some());
        assert!(game.get_unit(to.x, to.y).unwrap().is_some());
    }

    #[test]
    fn move_unit_into_content() {
        let (mut game, _, _) = test_game();
        let from = Point { x: 0, y: 0 };
        let to = Point { x: 1, y: 0 };
        assert!(game.move_unit(from, to).is_err());
        assert!(game.get_unit(from.x, from.y).unwrap().is_some());
        assert!(game.get_hex(to.x, to.y).unwrap().get_content().is_some());
    }

    #[test]
    fn move_unit_out_from_field() {
        let (mut game, _, _) = test_game();
        let from = Point { x: 0, y: 0 };
        let to = Point { x: 5, y: 6 };

        let result = game.move_unit(from, to);
        assert!(result.is_err());
        match result.unwrap_err().downcast_ref::<GameError>() {
            Some(GameError::NoHex) => {}
            _ => unreachable!("wrong error"),
        }
        assert!(game.get_unit(from.x, from.y).unwrap().is_some());
    }

    #[test]
    fn move_unit_from_hex_that_out_of_field() {
        let (mut game, _, _) = test_game();
        let from = Point { x: 5, y: 5 };
        let to = Point { x: 5, y: 6 };

        let result = game.move_unit(from, to);
        assert!(result.is_err());
        match result.unwrap_err().downcast_ref::<GameError>() {
            Some(GameError::NoHex) => {}
            _ => unreachable!("wrong error"),
        }
    }

    #[test]
    fn move_unit_from_hex_without_unit() {
        let (mut game, _, _) = test_game();
        let from = Point { x: 1, y: 0 };
        let to = Point { x: 0, y: 1 };

        let result = game.move_unit(from, to);
        assert!(result.is_err());
        match result.unwrap_err().downcast_ref::<GameError>() {
            Some(GameError::NoUnit) => {}
            _ => unreachable!("wrong error"),
        }
    }

    #[test]
    fn attack_unit_success_hurt() {
        let (mut game, unit, _) = test_game();
        let from = Point { x: 0, y: 0 };
        let to = Point { x: 1, y: 1 };
        // Customize unit by increasing it's damage
        let attacking_unit = Unit {
            damage: [unit.hp - 1, unit.hp - 1],
            ..unit
        };
        assert!(game
            .set_unit(from.x, from.y, Some(attacking_unit.clone()))
            .is_ok());

        let result = game.attack(from, to);

        assert!(result.is_ok());
        let (hurt, die) = result.unwrap();
        assert_eq!(hurt.len(), 1);
        assert_eq!(die.len(), 0);
        assert_eq!(
            hurt[0].unit.as_ref().unwrap().hp,
            unit.hp - attacking_unit.damage[0]
        );

        let hurt_unit = game.get_unit(to.x, to.y).unwrap().unwrap();
        assert_eq!(hurt_unit.hp, unit.hp - attacking_unit.damage[0]);
    }

    #[test]
    fn attack_unit_success_die_attack_more_then_hp() {
        let (mut game, unit, _) = test_game();
        let from = Point { x: 0, y: 0 };
        let to = Point { x: 1, y: 1 };
        // Customize unit by increasing it's damage
        let attacking_unit = Unit {
            damage: [unit.hp + 1, unit.hp + 1],
            ..unit
        };
        assert!(game.set_unit(from.x, from.y, Some(attacking_unit)).is_ok());

        let result = game.attack(from, to);
        assert!(result.is_ok());
        let (hurt, die) = result.unwrap();
        assert_eq!(hurt.len(), 0);
        assert_eq!(die.len(), 1);
        assert_eq!(die[0].unit.as_ref().unwrap().hp, 0);
    }

    #[test]
    fn attack_unit_success_die_attack_equals_hp() {
        let (mut game, unit, _) = test_game();
        let from = Point { x: 0, y: 0 };
        let to = Point { x: 1, y: 1 };
        // Customize unit by make it's damage equal to hp of another unit
        let attacking_unit = Unit {
            damage: [unit.hp, unit.hp],
            ..unit
        };
        assert!(game.set_unit(from.x, from.y, Some(attacking_unit)).is_ok());

        let result = game.attack(from, to);
        assert!(result.is_ok());
        let (hurt, die) = result.unwrap();
        assert_eq!(hurt.len(), 0);
        assert_eq!(die.len(), 1);
        assert_eq!(die[0].unit.as_ref().unwrap().hp, 0);
    }

    #[test]
    fn attack_from_empty_hex() {
        let (mut game, _, _) = test_game();
        let from = Point { x: 0, y: 1 };
        let to = Point { x: 1, y: 1 };
        let result = game.attack(from, to);

        assert!(result.is_err());
        match result.unwrap_err().downcast_ref::<GameError>() {
            Some(GameError::NoUnit) => {}
            _ => unreachable!("wrong error"),
        }
    }

    #[test]
    fn attack_to_empty_hex() {
        let (mut game, _, _) = test_game();
        let from = Point { x: 0, y: 0 };
        let to = Point { x: 0, y: 1 };
        let result = game.attack(from, to);

        assert!(result.is_err());
        match result.unwrap_err().downcast_ref::<GameError>() {
            Some(GameError::NoUnit) => {}
            _ => unreachable!("wrong error"),
        }
    }

    #[test]
    fn attack_from_hex_out_of_grid() {
        let (mut game, _, _) = test_game();
        let from = Point { x: 0, y: 10 };
        let to = Point { x: 1, y: 1 };
        let result = game.attack(from, to);

        assert!(result.is_err());
        match result.unwrap_err().downcast_ref::<GameError>() {
            Some(GameError::NoHex) => {}
            _ => unreachable!("wrong error"),
        }
    }

    #[test]
    fn attack_to_hex_out_of_grid() {
        let (mut game, _, _) = test_game();
        let from = Point { x: 0, y: 0 };
        let to = Point { x: 0, y: 10 };
        let result = game.attack(from, to);

        assert!(result.is_err());
        match result.unwrap_err().downcast_ref::<GameError>() {
            Some(GameError::NoHex) => {}
            _ => unreachable!("wrong error"),
        }
    }

    // For tests with pathfinding we need match larger field

    // 0   | U |   |   |   |   |   |   |    0
    // 1     | U |   |   |   |   |   |   |  1
    // 2   |   |   |   |   | U |   | W |    2
    // 3     |   |   |   |   |   |   |   |  3
    // 4   |   |   |   |   |   |   |   |    4
    // 5     |   | W |   |   |   |   |   |  5
    // 6   |   |   |   |   |   |   |   |    6
    // 7     |   |   |   |   | W | W |   |  7
    // 8   |   |   |   |   | W | U | W |    8
    // 9     |   | U |   |   | W | W | W |  9
    // 10  |   |   |   |   |   |   | W |    10
    // 11    |   |   |   |   |   | W |   |  11

    fn test_big_game() -> Game {
        let mut game = Game::new(8, 12);
        let unit = Unit::new(1, 5, [5, 5], 3);
        let wall = Wall {};
        assert!(game.set_unit(0, 0, Some(unit.clone())).is_ok());
        assert!(game.set_unit(1, 1, Some(unit.clone())).is_ok());
        assert!(game.set_unit(4, 2, Some(unit.clone())).is_ok());
        assert!(game.set_unit(5, 8, Some(unit.clone())).is_ok());
        assert!(game.set_unit(2, 9, Some(unit.clone())).is_ok());
        assert!(game.set_content(6, 2, Some(Content::Wall(wall))).is_ok());
        assert!(game.set_content(2, 5, Some(Content::Wall(wall))).is_ok());
        assert!(game.set_content(5, 7, Some(Content::Wall(wall))).is_ok());
        assert!(game.set_content(6, 7, Some(Content::Wall(wall))).is_ok());
        assert!(game.set_content(4, 8, Some(Content::Wall(wall))).is_ok());
        assert!(game.set_content(6, 8, Some(Content::Wall(wall))).is_ok());
        assert!(game.set_content(5, 9, Some(Content::Wall(wall))).is_ok());
        assert!(game.set_content(6, 9, Some(Content::Wall(wall))).is_ok());
        assert!(game.set_content(7, 9, Some(Content::Wall(wall))).is_ok());
        assert!(game.set_content(6, 10, Some(Content::Wall(wall))).is_ok());
        assert!(game.set_content(6, 11, Some(Content::Wall(wall))).is_ok());

        game
    }

    #[test]
    fn find_neighbours_in_the_middle() {
        let game = test_big_game();
        // hex with nothing nearby for odd y
        let hex = game.get_hex(1, 9).unwrap();

        let neighbours = game.find_neighbours(&hex.to_point());
        // convert to points to simplify assertion
        let neighbours: Vec<Point> = neighbours.iter().map(|hex| hex.to_point()).collect();
        assert_eq!(neighbours.len(), 6);
        assert!(neighbours.contains(&Point { x: 0, y: 8 }));
        assert!(neighbours.contains(&Point { x: 1, y: 8 }));
        assert!(neighbours.contains(&Point { x: 0, y: 9 }));
        assert!(neighbours.contains(&Point { x: 2, y: 9 }));
        assert!(neighbours.contains(&Point { x: 0, y: 10 }));
        assert!(neighbours.contains(&Point { x: 1, y: 10 }));

        // hex with nothing nearby for even y
        let hex = game.get_hex(1, 10).unwrap();

        let neighbours = game.find_neighbours(&hex.to_point());
        // convert to points to simplify assertion
        let neighbours: Vec<Point> = neighbours.iter().map(|hex| hex.to_point()).collect();
        assert_eq!(neighbours.len(), 6);
        assert!(neighbours.contains(&Point { x: 1, y: 9 }));
        assert!(neighbours.contains(&Point { x: 2, y: 9 }));
        assert!(neighbours.contains(&Point { x: 0, y: 10 }));
        assert!(neighbours.contains(&Point { x: 2, y: 10 }));
        assert!(neighbours.contains(&Point { x: 1, y: 11 }));
        assert!(neighbours.contains(&Point { x: 2, y: 11 }));
    }

    #[test]
    fn find_neighbours_on_boarder() {
        let game = test_big_game();
        // hex near boarder
        let hex = game.get_hex(1, 0).unwrap();

        let neighbours = game.find_neighbours(&hex.to_point());
        // convert to points to simplify assertion
        let neighbours: Vec<Point> = neighbours.iter().map(|hex| hex.to_point()).collect();
        assert_eq!(neighbours.len(), 4);
        assert!(neighbours.contains(&Point { x: 0, y: 0 }));
        assert!(neighbours.contains(&Point { x: 2, y: 0 }));
        assert!(neighbours.contains(&Point { x: 1, y: 1 }));
        assert!(neighbours.contains(&Point { x: 2, y: 1 }));
    }

    #[test]
    fn find_neighbours_in_even_corner() {
        let game = test_big_game();
        let hex = game.get_hex(0, 0).unwrap();

        let neighbours = game.find_neighbours(&hex.to_point());
        // convert to points to simplify assertion
        let neighbours: Vec<Point> = neighbours.iter().map(|hex| hex.to_point()).collect();
        assert_eq!(neighbours.len(), 3);
        assert!(neighbours.contains(&Point { x: 1, y: 0 }));
        assert!(neighbours.contains(&Point { x: 0, y: 1 }));
        assert!(neighbours.contains(&Point { x: 1, y: 1 }));
    }

    #[test]
    fn find_neighbours_in_odd_corner() {
        let game = test_big_game();
        let hex = game.get_hex(0, 11).unwrap();

        let neighbours = game.find_neighbours(&hex.to_point());
        // convert to points to simplify assertion
        let neighbours: Vec<Point> = neighbours.iter().map(|hex| hex.to_point()).collect();
        assert_eq!(neighbours.len(), 2);
        assert!(neighbours.contains(&Point { x: 0, y: 10 }));
        assert!(neighbours.contains(&Point { x: 1, y: 11 }));
    }

    #[test]
    fn available_points_no_hex() {
        let game = test_big_game();

        let points = game.available_points(&None);
        assert!(points.is_empty());
    }

    #[test]
    fn available_points_no_unit() {
        let game = test_big_game();
        let hex = game.get_hex(5, 5);

        let points = game.available_points(&hex);
        assert!(points.is_empty());
    }

    #[test]
    fn available_points_for_unit_without_movements() {
        let game = test_big_game();
        let mut hex = game.get_hex(0, 0).unwrap();
        let unit = hex.get_unit_mut().unwrap();
        unit.change_movements(unit.speed);

        let points = game.available_points(&Some(hex));
        assert_eq!(points.len(), 1);
        assert!(points.contains(&hex.to_point()));
    }

    #[test]
    fn available_points_for_unit_with_no_obstacle() {
        let game = test_big_game();
        let mut hex = game.get_hex(2, 9).unwrap();
        let unit = hex.get_unit_mut().unwrap();
        unit.change_movements(unit.speed - 1);

        let points = game.available_points(&Some(hex));
        assert_eq!(points.len(), 7);
        assert!(points.contains(&Point { x: 1, y: 8 }));
        assert!(points.contains(&Point { x: 2, y: 8 }));
        assert!(points.contains(&Point { x: 1, y: 9 }));
        assert!(points.contains(&Point { x: 2, y: 9 }));
        assert!(points.contains(&Point { x: 3, y: 9 }));
        assert!(points.contains(&Point { x: 1, y: 10 }));
        assert!(points.contains(&Point { x: 2, y: 10 }));
    }

    #[test]
    fn available_points_for_unit_in_corner_with_other_unit_nearby() {
        let game = test_big_game();
        let hex = game.get_hex(0, 0);

        let points = game.available_points(&hex);
        assert_eq!(points.len(), 12);
        assert!(points.contains(&Point { x: 0, y: 0 }));
        assert!(points.contains(&Point { x: 1, y: 0 }));
        assert!(points.contains(&Point { x: 2, y: 0 }));
        assert!(points.contains(&Point { x: 3, y: 0 }));
        assert!(points.contains(&Point { x: 0, y: 1 }));
        assert!(points.contains(&Point { x: 2, y: 1 }));
        assert!(points.contains(&Point { x: 3, y: 1 }));
        assert!(points.contains(&Point { x: 1, y: 2 }));
        assert!(points.contains(&Point { x: 1, y: 2 }));
        assert!(points.contains(&Point { x: 2, y: 2 }));
        assert!(points.contains(&Point { x: 0, y: 3 }));
        assert!(points.contains(&Point { x: 1, y: 3 }));
    }

    #[test]
    fn available_points_unit_interacts_with_wall() {
        let game = test_big_game();
        let hex = game.get_hex(4, 2);

        let points = game.available_points(&hex);
        assert!(!points.contains(&Point { x: 7, y: 2 }));
    }

    #[test]
    fn available_points_unit_surrendered() {
        let game = test_big_game();
        let hex = game.get_hex(5, 8);

        let points = game.available_points(&hex);
        assert_eq!(points.len(), 1);
        assert!(points.contains(&Point { x: 5, y: 8 }));
    }

    #[test]
    fn available_points_unit_speed_greaer_then_field() {
        let game = test_big_game();
        let mut hex = game.get_hex(2, 9).unwrap();
        let mut unit = hex.get_unit_mut().unwrap();
        unit.movements = 1000;

        let points = game.available_points(&Some(hex));

        assert_eq!(points.len(), 79);
    }

    fn hexmap() -> HashMap<Point, u32> {
        let mut hexmap: HashMap<Point, u32> = HashMap::new();

        // path
        hexmap.insert(Point { x: 0, y: 4 }, 0);
        hexmap.insert(Point { x: 1, y: 5 }, 1);
        hexmap.insert(Point { x: 1, y: 6 }, 2);
        hexmap.insert(Point { x: 2, y: 6 }, 3);
        hexmap.insert(Point { x: 3, y: 5 }, 4);
        hexmap.insert(Point { x: 3, y: 4 }, 5);

        // additional
        hexmap.insert(Point { x: 1, y: 3 }, 1);
        hexmap.insert(Point { x: 1, y: 4 }, 1);
        hexmap.insert(Point { x: 0, y: 5 }, 1);
        hexmap.insert(Point { x: 3, y: 7 }, 3);
        hexmap.insert(Point { x: 1, y: 8 }, 4);
        hexmap.insert(Point { x: 0, y: 8 }, 5);

        hexmap
    }

    fn assert_are_neighbours(game: &Game, point1: &Point, point2: &Point) {
        assert!(game
            .find_neighbours(point1)
            .iter()
            .map(|hex| hex.to_point())
            .any(|point| point == *point2))
    }

    #[test]
    fn restore_path_from_hexmap_correct_path() {
        let map = hexmap();
        let game = test_big_game();
        let start_point = Point { x: 0, y: 4 };
        let end_point = Point { x: 3, y: 4 };

        let path = game.restore_path_from_hexmap(start_point, end_point, &map);
        assert!(path.is_ok());
        let path = path.unwrap();
        assert_eq!(path.len(), 6);

        // check if previouse point in result is neighbour of current
        for (i, point) in path.iter().enumerate() {
            if i == 0 {
                // skip first one because it has no previous
                continue;
            }

            assert_are_neighbours(&game, point, &path[i - 1]);
        }

        assert_eq!(*path.first().unwrap(), start_point);
        assert_eq!(*path.last().unwrap(), end_point);
    }

    #[test]
    fn restore_path_from_hexmap_no_start_hex() {
        let mut map = hexmap();
        let game = test_big_game();
        let start_point = Point { x: 0, y: 4 };
        let end_point = Point { x: 3, y: 4 };

        map.remove(&start_point);

        let path = game.restore_path_from_hexmap(start_point, end_point, &map);
        assert!(path.is_err());
        assert_eq!(
            GameError::NoHex,
            *path.unwrap_err().downcast_ref::<GameError>().unwrap()
        );
    }

    #[test]
    fn restore_path_from_hexmap_no_finish_hex() {
        let mut map = hexmap();
        let game = test_big_game();
        let start_point = Point { x: 0, y: 4 };
        let end_point = Point { x: 3, y: 4 };

        map.remove(&end_point);

        let path = game.restore_path_from_hexmap(start_point, end_point, &map);
        assert!(path.is_err());
        assert_eq!(
            GameError::NoHex,
            *path.unwrap_err().downcast_ref::<GameError>().unwrap()
        );
    }

    #[test]
    fn restore_path_from_hexmap_empty_hexmap() {
        let map: HashMap<Point, u32> = HashMap::new();
        let game = test_big_game();
        let start_point = Point { x: 0, y: 4 };
        let end_point = Point { x: 3, y: 4 };

        let path = game.restore_path_from_hexmap(start_point, end_point, &map);
        assert!(path.is_err());
        assert_eq!(
            GameError::NoHex,
            *path.unwrap_err().downcast_ref::<GameError>().unwrap()
        );
    }

    #[test]
    fn restore_path_from_hexmap_discontinuous() {
        let mut map = hexmap();
        let game = test_big_game();
        let start_point = Point { x: 0, y: 4 };
        let end_point = Point { x: 3, y: 4 };

        map.remove(&Point { x: 3, y: 5 });

        let path = game.restore_path_from_hexmap(start_point, end_point, &map);
        assert!(path.is_err());
        assert_eq!(
            GameError::NoHex,
            *path.unwrap_err().downcast_ref::<GameError>().unwrap()
        );
    }

    #[test]
    fn restore_path_from_hexmap_from_equals_to() {
        let map = hexmap();
        let game = test_big_game();
        let start_point = Point { x: 0, y: 4 };

        let path = game.restore_path_from_hexmap(start_point, start_point, &map);
        assert!(path.is_ok());
        let path = path.unwrap();
        assert_eq!(path.len(), 1);
        assert_eq!(path[0], start_point);
    }

    #[test]
    fn attack_unit_far_away() {
        let mut game = test_big_game();
        let from = Point { x: 0, y: 0 };
        let to = Point { x: 4, y: 2 };
        let result = game.attack(from, to);

        assert!(result.is_err());
        match result.unwrap_err().downcast_ref::<GameError>() {
            Some(GameError::WrongHex) => {}
            _ => unreachable!("not neighbour"),
        }
    }
}
