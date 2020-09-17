use crate::api::common::Point;
use crate::game_objects::grid::Grid;
use crate::game_objects::hex::Hex;
use crate::game_objects::hex_objects::content::Content;
use crate::game_objects::unit::Unit;

use eyre::{Result, WrapErr};
use rand::Rng;
use serde::Serialize;
use thiserror::Error;
use tracing::instrument;

#[derive(Serialize, Debug)]
pub struct Game {
    cmd: String,
    num_x: u32,
    num_y: u32,
    field: Grid,
}

#[derive(Error, Debug)]
pub enum GameError {
    #[error("no hex")]
    NoHex,

    #[error("no unit")]
    NoUnit,
}

impl Game {
    // Public api
    pub fn new(num_x: u32, num_y: u32) -> Game {
        Game {
            cmd: String::from("field"),
            num_x,
            num_y,
            field: Grid::new(num_x, num_y),
        }
    }

    #[instrument(skip(self))]
    pub fn move_unit(&mut self, from: Point, to: Point) -> Result<Vec<Point>> {
        match self
            .get_unit(from.x, from.y)
            .wrap_err("failed to move unit; failed to get unit")?
        {
            Some(unit) => {
                self.set_unit(from.x, from.y, None)
                    .wrap_err("failed to move unit; failed to unset unit")?;
                self.set_unit(to.x, to.y, Some(unit))
                    .wrap_err("failed to move unit; failed to set unit")?;
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

        let to_hex = match self.get_hex(to.x, to.y) {
            Some(hex) => hex,
            None => Err(GameError::NoHex).wrap_err("failed to attack to")?,
        };

        let to_unit = match to_hex.get_unit_mut() {
            Some(unit) => unit,
            None => Err(GameError::NoUnit).wrap_err("failed to attack to")?,
        };

        let mut rng = rand::thread_rng();
        let dmg = rng.gen_range(from_unit.damage[0], from_unit.damage[1]);
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
    // TODO: remvoe pub after creating new games from presets
    #[instrument(skip(self))]
    pub fn set_unit(&mut self, x: u32, y: u32, unit: Option<Unit>) -> Result<()> {
        match self.get_hex(x, y) {
            Some(hex) => {
                hex.set_unit(unit);
                Ok(())
            }
            None => Err(GameError::NoHex).wrap_err("failed to set unit")?,
        }
    }

    #[instrument(skip(self))]
    fn get_unit(&mut self, x: u32, y: u32) -> Result<Option<Unit>> {
        match self.get_hex(x, y) {
            Some(hex) => Ok(hex.get_unit()),
            None => Err(GameError::NoHex).wrap_err("failed to get unit")?,
        }
    }

    // Hex stuff
    #[instrument(skip(self))]
    fn get_hex(&mut self, x: u32, y: u32) -> Option<&mut Hex> {
        self.field.get_hex(x, y)
    }

    // Content stuff
    // TODO: remvoe pub after creating new games from presets
    #[instrument(skip(self))]
    pub fn set_content(&mut self, x: u32, y: u32, content: Option<Content>) -> Result<()> {
        match self.get_hex(x, y) {
            Some(hex) => {
                hex.set_content(content);
                Ok(())
            }
            None => Err(GameError::NoHex).wrap_err("failed to set content")?,
        }
    }
}

#[cfg(test)]
mod test {
    use super::super::game_objects::hex_objects::wall::Wall;
    use super::*;

    #[test]
    fn new() {
        let num_x = 8;
        let num_y = 3;
        let game = Game::new(num_x, num_y);

        assert_eq!(game.num_x, num_x);
        assert_eq!(game.num_y, num_y);
        assert_eq!(game.cmd, String::from("field"));
    }

    #[test]
    fn get_hex() {
        let mut game = Game::new(2, 2);

        let hex = game.get_hex(0, 1);
        assert!(hex.is_some());

        let hex = hex.unwrap();
        assert_eq!(hex.x, 0);
        assert_eq!(hex.y, 1);

        let hex = game.get_hex(5, 6);
        assert!(hex.is_none());
    }

    #[test]
    fn set_get_unit() {
        let mut game = Game::new(2, 2);
        let unit = Unit {
            player: 1,
            hp: 10,
            damage: [2, 3],
            speed: 3,
        };

        assert!(game.set_unit(0, 1, Some(unit.clone())).is_ok());
        assert!(game.set_unit(1, 0, None).is_ok());

        assert!(game.get_unit(0, 1).unwrap().is_some());
        assert!(game.get_unit(1, 0).unwrap().is_none());
        assert!(game.get_unit(0, 0).unwrap().is_none());

        // TODO: how to check if it is particulary NoHex error
        assert!(game.set_unit(4, 5, Some(unit.clone())).is_err());
        assert!(game.get_unit(4, 5).is_err());
    }

    #[test]
    fn set_content() {
        let mut game = Game::new(2, 2);
        let wall = Wall {};
        assert!(game.set_content(0, 1, Some(Content::Wall(wall))).is_ok());
    }

    // TODO: WIP tests
    #[test]
    fn move_unit() {}

    #[test]
    fn move_no_unit() {}

    #[test]
    fn move_no_hex_from() {}

    #[test]
    fn move_no_hex_to() {}

    #[test]
    fn attack_unit_hurt() {}

    #[test]
    fn attack_unit_die() {}

    #[test]
    fn attack_no_unit() {}

    #[test]
    fn attack_no_unit_to() {}

    #[test]
    fn attack_no_hex_from() {}

    #[test]
    fn attack_no_hex_to() {}

    // #[test]
    // fn set_unit() {
    //     // coords of existing hex
    //     let x1 = 0;
    //     let y1 = 1;
    //     // coords of non existing hex
    //     let x2 = 10;
    //     let y2 = 18;
    //     // unit
    //     let player = 1;
    //     let hp = 1;
    //     let damage = [1, 2];
    //     let speed = 1;

    //     let mut game = Game::new(5, 5);
    //     let unit = Unit {
    //         player,
    //         hp,
    //         damage,
    //         speed,
    //     };

    //     let res = game.set_unit(x1, y1, Some(unit.clone()));
    //     assert!(res.is_ok());
    //     assert!(game.field.get_hex(x1, y1).unwrap().unit.is_some());
    //     assert!(game.field.get_hex(x1, y1).unwrap().content.is_none());

    //     let field_unit = game.field.get_hex(x1, y1).unwrap().unit.as_ref().unwrap();
    //     assert_eq!(field_unit.player, player);
    //     assert_eq!(field_unit.hp, hp);
    //     assert_eq!(field_unit.damage, damage);
    //     assert_eq!(field_unit.speed, speed);

    //     let res = game.set_unit(x2, y2, Some(unit));
    //     assert!(res.is_err());
    //     assert_eq!(res.unwrap_err(), "Error while setting unit: no hex");
    // }

    // #[test]
    // fn set_content() {
    //     // coords of existing hex
    //     let x1 = 0;
    //     let y1 = 1;
    //     // coords of non existing hex
    //     let x2 = 10;
    //     let y2 = 18;

    //     let mut game = Game::new(5, 5);
    //     let wall = Wall {};

    //     let res = game.set_content(x1, y1, Some(Content::Wall(wall.clone())));
    //     assert!(res.is_ok());
    //     assert!(game.field.get_hex(x1, y1).unwrap().unit.is_none());
    //     assert!(game.field.get_hex(x1, y1).unwrap().content.is_some());

    //     let res = game.set_content(x2, y2, Some(Content::Wall(wall)));
    //     assert!(res.is_err());
    //     assert_eq!(res.unwrap_err(), "Error while setting content: no hex");
    // }

    // #[test]
    // fn serialize() {
    //     let num_x = 1;
    //     let num_y = 1;
    //     let game = Game::new(num_x, num_y);

    //     let game_string = serde_json::to_string(&game).unwrap();
    //     assert_eq!(
    //         game_string,
    //         format!(
    //             "{{\"cmd\":\"field\",\"num_x\":1,\"num_y\":1,\"field\":{}}}",
    //             serde_json::to_string(&game.field).unwrap()
    //         ),
    //     );
    // }
}
