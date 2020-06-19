use super::game_objects::grid::Grid;
use super::game_objects::hex_objects::content::Content;
use super::game_objects::unit::Unit;

use serde::Serialize;

#[derive(Serialize)]
pub struct Game {
    row_n: u32,
    col_n: u32,
    field: Grid,
}

impl Game {
    pub fn new(row_n: u32, col_n: u32) -> Game {
        Game {
            row_n,
            col_n,
            field: Grid::new(row_n, col_n),
        }
    }

    pub fn set_unit(&mut self, x: u32, y: u32, unit: Unit) -> Result<(), &str> {
        match self.field.get_hex(x, y) {
            Some(hex) => {
                hex.unit = Some(unit);
                Ok(())
            }
            None => Err("no hex"),
        }
    }

    pub fn set_content(&mut self, x: u32, y: u32, content: Content) -> Result<(), &str> {
        match self.field.get_hex(x, y) {
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

        let res = game.set_unit(x1, y1, unit.clone());
        assert!(res.is_ok());
        assert!(game.field.get_hex(x1, y1).unwrap().unit.is_some());
        assert!(game.field.get_hex(x1, y1).unwrap().content.is_none());

        let field_unit = game.field.get_hex(x1, y1).unwrap().unit.as_ref().unwrap();
        assert_eq!(field_unit.player, player);
        assert_eq!(field_unit.hp, hp);
        assert_eq!(field_unit.attack, attack);
        assert_eq!(field_unit.speed, speed);

        let res = game.set_unit(x2, y2, unit);
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

        let res = game.set_content(x1, y1, Content::Wall(wall.clone()));
        assert!(res.is_ok());
        assert!(game.field.get_hex(x1, y1).unwrap().unit.is_none());
        assert!(game.field.get_hex(x1, y1).unwrap().content.is_some());

        let res = game.set_content(x2, y2, Content::Wall(wall));
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
                "{{\"row_n\":1,\"col_n\":1,\"field\":{}}}",
                serde_json::to_string(&game.field).unwrap()
            ),
        );
    }

    #[test]
    fn my_test() {
        let mut game = Game::new(3, 2);
        let zombie = Unit {
            player: 1,
            hp: 20,
            attack: [2, 3],
            speed: 1
        };
        let wolf = Unit {
            player: 2,
            hp: 12,
            attack: [3, 5],
            speed: 2
        };
        let mut res = game.set_content(1, 0, Content::Wall(Wall {}));
        println!("{}", res.is_ok());
        res = game.set_unit(0, 0, wolf);
        println!("{}", res.is_ok());
        res = game.set_unit(2, 1, zombie);
        println!("{}", res.is_ok());
        let game_string = serde_json::to_string(&game).unwrap();
        println!("{}", game_string);
        let hex = game.field.get_hex(2, 1).unwrap();
        let z = hex.unit.clone();
        let z_string = serde_json::to_string(&z).unwrap();
        println!("{}", z_string);
        let hex_string = serde_json::to_string(&hex).unwrap();
        println!("{}", hex_string);
    }
}
