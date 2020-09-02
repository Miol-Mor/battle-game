use super::game_objects::grid::Grid;
use super::game_objects::hex_objects::content::Content;
use super::game_objects::unit::Unit;

use crate::api::common::Point;

use serde::Serialize;

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

    pub fn set_unit(&mut self, x: u32, y: u32, unit: Unit) -> Result<(), &str> {
        match self.field.get_hex(x, y) {
            Some(hex) => {
                hex.unit = Some(unit);
                Ok(())
            }
            None => Err("Error while setting unit: no hex"),
        }
    }

    pub fn set_content(&mut self, x: u32, y: u32, content: Content) -> Result<(), &str> {
        match self.field.get_hex(x, y) {
            Some(hex) => {
                hex.content = Some(content);
                Ok(())
            }
            None => Err("Error while setting content: no hex"),
        }
    }

    pub fn move_unit(&mut self, from: Point, to: Point) -> Result<Vec<Point>, &str> {
        // something like check
        Ok(vec![from, to])
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
