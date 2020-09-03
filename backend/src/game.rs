use super::game_objects::grid::Grid;
use super::game_objects::hex_objects::content::Content;
use super::game_objects::hex::Hex;
use super::game_objects::unit::Unit;

use crate::api::common::Point;

use serde::Serialize;

use rand::Rng;

#[derive(Serialize, Debug)]
pub struct Game {
    cmd: String,
    num_x: u32,
    num_y: u32,
    field: Grid,
}

impl Game {
    pub fn new(num_x: u32, num_y: u32) -> Game {
        Game {
            cmd: String::from("field"),
            num_x,
            num_y,
            field: Grid::new(num_x, num_y),
        }
    }

    pub fn get_hex(&mut self, x: u32, y: u32) -> Result<&mut Hex, &str> {
        match self.field.get_hex(x, y) {
            Some(hex) => {
                Ok(hex)
            }
            None => Err("Error while getting hex: no such hex"),
        }
    }

    pub fn set_unit(&mut self, x: u32, y: u32, unit: Option<Unit>) -> Result<(), &str> {
        match self.field.get_hex(x, y) {
            Some(hex) => {
                hex.unit = unit;
                Ok(())
            }
            None => Err("Error while setting unit: no hex"),
        }
    }

    fn get_unit(&mut self, x: u32, y: u32) -> Result<Option<Unit>, &str> {
        match self.field.get_hex(x, y) {
            Some(hex) => Ok(hex.unit.clone()),
            None => Err("Error while getting unit: no such hex"),
        }
    }

    pub fn set_content(&mut self, x: u32, y: u32, content: Content) -> Result<(), &str> {
        match self.field.get_hex(x, y) {
            Some(hex) => Ok(()),
            None => Err("Error while setting content: no hex"),
        }
    }

    pub fn move_unit(&mut self, from: Point, to: Point) -> Result<Vec<Point>, &str> {
        match self.get_unit(from.x, from.y).unwrap() {
            Some(unit) => {
                self.set_unit(from.x, from.y, None);
                self.set_unit(to.x, to.y, Some(unit));
                Ok(vec![from, to])
            }
            None => Err("No unit found"),
        }
    }

    pub fn attack(&mut self, from: Point, to: Point) -> Result<(Vec<Hex>, Vec<Hex>), &str> {
        let mut from = self.get_hex(from.x, from.y).unwrap();
        let from_unit = match from.unit {
            None => return Err("No unit found in 'from' hex"),
            Some(unit) => unit,
        };

        let mut to = self.get_hex(to.x, to.y).unwrap();
        let mut to_unit = match to.unit {
            None => return Err("No unit found in 'to' hex"),
            Some(unit) => unit,
        };

        let mut hurt: Vec<Hex> = vec![];
        let mut die: Vec<Hex> = vec![];

        let mut rng = rand::thread_rng();
        let dmg = rng.gen_range(from_unit.damage[0], from_unit.damage[1]);
        to_unit.hp -= dmg;
        if to_unit.hp <= 0 {
            die.push(*to);
        } else {
            hurt.push(*to);
        }
        Ok((hurt, die))
    }
}

#[cfg(test)]
mod test {
    use super::super::game_objects::hex_objects::content::Content;
    use super::super::game_objects::hex_objects::wall::Wall;
    use super::super::game_objects::unit::Unit;
    use super::Game;

    #[test]
    fn new() {
        let num_x = 8;
        let num_y = 3;
        let game = Game::new(num_x, num_y);

        assert_eq!(game.num_x, num_x);
        assert_eq!(game.num_y, num_y);
    }

    #[test]
    fn set_unit() {
        // coords of existing hex
        let x1 = 0;
        let y1 = 1;
        // coords of non existing hex
        let x2 = 10;
        let y2 = 18;
        // unit
        let player = 1;
        let hp = 1;
        let damage = [1, 2];
        let speed = 1;

        let mut game = Game::new(5, 5);
        let unit = Unit {
            player,
            hp,
            damage,
            speed,
        };

        let res = game.set_unit(x1, y1, unit.clone());
        assert!(res.is_ok());
        assert!(game.field.get_hex(x1, y1).unwrap().unit.is_some());
        assert!(game.field.get_hex(x1, y1).unwrap().content.is_none());

        let field_unit = game.field.get_hex(x1, y1).unwrap().unit.as_ref().unwrap();
        assert_eq!(field_unit.player, player);
        assert_eq!(field_unit.hp, hp);
        assert_eq!(field_unit.damage, damage);
        assert_eq!(field_unit.speed, speed);

        let res = game.set_unit(x2, y2, unit);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "Error while setting unit: no hex");
    }

    #[test]
    fn set_content() {
        // coords of existing hex
        let x1 = 0;
        let y1 = 1;
        // coords of non existing hex
        let x2 = 10;
        let y2 = 18;

        let mut game = Game::new(5, 5);
        let wall = Wall {};

        let res = game.set_content(y1, x1, Content::Wall(wall.clone()));
        assert!(res.is_ok());
        assert!(game.field.get_hex(y1, x1).unwrap().unit.is_none());
        assert!(game.field.get_hex(y1, x1).unwrap().content.is_some());

        let res = game.set_content(x2, y2, Content::Wall(wall));
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "Error while setting content: no hex");
    }

    #[test]
    fn serialize() {
        let num_x = 1;
        let num_y = 1;
        let game = Game::new(num_x, num_y);

        let game_string = serde_json::to_string(&game).unwrap();
        assert_eq!(
            game_string,
            format!(
                "{{\"cmd\":\"field\",\"num_x\":1,\"num_y\":1,\"field\":{}}}",
                serde_json::to_string(&game.field).unwrap()
            ),
        );
    }
}
