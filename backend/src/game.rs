use super::game_objects::grid::Grid;
use super::game_objects::hex_objects::content::Content;
use super::game_objects::unit::Unit;

use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct Game {
    cmd: String,
    row_n: u32,
    col_n: u32,
    field: Grid,
}

impl Game {
    pub fn new(row_n: u32, col_n: u32) -> Game {
        Game {
            cmd: String::from("field"),
            row_n,
            col_n,
            field: Grid::new(row_n, col_n),
        }
    }

    pub fn set_unit(&mut self, y: u32, x: u32, unit: Unit) -> Result<(), &str> {
        match self.field.get_hex(y, x) {
            Some(hex) => {
                hex.unit = Some(unit);
                Ok(())
            }
            None => Err("no hex"),
        }
    }

    pub fn set_content(&mut self, y: u32, x: u32, content: Content) -> Result<(), &str> {
        match self.field.get_hex(y, x) {
            Some(hex) => {
                hex.content = Some(content);
                Ok(())
            }
            None => Err("no hex"),
        }
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
        let row_n = 3;
        let col_n = 8;
        let game = Game::new(row_n, col_n);

        assert_eq!(game.row_n, row_n);
        assert_eq!(game.col_n, col_n);
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
        let attack = [1, 2];
        let speed = 1;

        let mut game = Game::new(5, 5);
        let unit = Unit {
            player,
            hp,
            attack,
            speed,
        };

        let res = game.set_unit(y1, x1, unit.clone());
        assert!(res.is_ok());
        assert!(game.field.get_hex(y1, x1).unwrap().unit.is_some());
        assert!(game.field.get_hex(y1, x1).unwrap().content.is_none());

        let field_unit = game.field.get_hex(y1, x1).unwrap().unit.as_ref().unwrap();
        assert_eq!(field_unit.player, player);
        assert_eq!(field_unit.hp, hp);
        assert_eq!(field_unit.attack, attack);
        assert_eq!(field_unit.speed, speed);

        let res = game.set_unit(y2, x2, unit);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "no hex");
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

        let res = game.set_content(y2, x2, Content::Wall(wall));
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "no hex");
    }

    #[test]
    fn serialize() {
        let row_n = 1;
        let col_n = 1;
        let game = Game::new(row_n, col_n);

        let game_string = serde_json::to_string(&game).unwrap();
        assert_eq!(
            game_string,
            format!(
                "{{\"cmd\":\"field\",\"row_n\":1,\"col_n\":1,\"field\":{}}}",
                serde_json::to_string(&game.field).unwrap()
            ),
        );
    }
}
