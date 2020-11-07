use crate::api::common::Point;
use crate::game_objects::grid::Grid;
use crate::game_objects::hex::Hex;
use crate::game_objects::hex_objects::content::Content;
use crate::game_objects::unit::Unit;

use eyre::{Result, WrapErr};
use rand::Rng;
use thiserror::Error;
use tracing::instrument;

#[derive(Debug, Clone)]
pub struct Game {
    pub field: Grid,
    pub selected_hex: Option<Hex>,
}

#[derive(Error, Debug)]
pub enum GameError {
    #[error("no hex")]
    NoHex,

    #[error("no unit")]
    NoUnit,

    #[error("no moves")]
    NoMoves,
}

#[derive(Debug)]
pub struct Selection {
    pub target: Point,
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
    // return target and vector of hexes to highlight
    pub fn select_unit(&mut self, target: Point) -> Result<Selection> {
        match self
            .get_unit(target.x, target.y)
            .wrap_err("failed to select unit; failed to get unit")?
        {
            Some(_) => {
                self.selected_hex = self.get_hex(target.x, target.y);
                Ok(Selection {
                    target,
                    highlight_hexes: vec![],
                })
            }
            None => Err(GameError::NoUnit).wrap_err("failed to get unit on select")?,
        }
    }

    #[instrument(skip(self))]
    // return target and vector of hexes to highlight
    pub fn deselect_unit(&mut self) {
        self.selected_hex = None
    }

    #[instrument(skip(self))]
    pub fn move_unit(&mut self, from: Point, to: Point) -> Result<Vec<Point>> {
        match self.get_unit(from.x, from.y).wrap_err("get unit")? {
            Some(unit) => {
                if unit.movements == 0 {
                    return Err(GameError::NoMoves).wrap_err("no moves left")?;
                }
                let mut unit_mut = unit;
                unit_mut.change_movements(unit.speed);
                self.set_unit(to.x, to.y, Some(unit_mut))
                    .wrap_err("failed to move unit; failed to set unit")?;
                self.set_unit(from.x, from.y, None)
                    .wrap_err("failed to move unit; failed to unset unit")?;
                Ok(vec![from, to])
            }
            None => Err(GameError::NoUnit).wrap_err("failed to get unit on move")?,
        }
    }

    #[instrument(skip(self))]
    pub fn attack(&mut self, from: Point, to: Point) -> Result<(Vec<Hex>, Vec<Hex>)> {
        let from_hex = match self.get_hex(from.x, from.y) {
            Some(hex) => hex,
            None => Err(GameError::NoHex).wrap_err("failed to attack from")?,
        };

        let from_unit = match from_hex.get_unit() {
            Some(unit) => unit,
            None => Err(GameError::NoUnit).wrap_err("failed to attack from")?,
        };

        let to_hex = match self.get_hex_mut(to.x, to.y) {
            Some(hex) => hex,
            None => Err(GameError::NoHex).wrap_err("failed to attack to")?,
        };

        let to_unit = match to_hex.get_unit_mut() {
            Some(unit) => unit,
            None => Err(GameError::NoUnit).wrap_err("failed to attack to")?,
        };

        let mut rng = rand::thread_rng();
        let dmg = rng.gen_range(from_unit.damage[0], from_unit.damage[1] + 1);
        to_unit.change_hp(-(dmg as i32));

        let mut hurt: Vec<Hex> = vec![];
        let mut die: Vec<Hex> = vec![];
        if to_unit.hp == 0 {
            die.push(to_hex.clone());
        } else {
            hurt.push(to_hex.clone());
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
            let mut unit = hex.get_unit_mut().unwrap();
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
}

#[cfg(test)]
mod test {
    use super::super::game_objects::hex_objects::wall::Wall;
    use super::*;

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
        assert!(game.set_content(0, 1, Some(Content::Wall(wall))).is_ok());

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
}
