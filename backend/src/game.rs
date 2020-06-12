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
    use super::Game;
    use crate::fixtures;

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
        let mut game = Game::new(fixtures::grid::row_n(), fixtures::grid::col_n());
        let unit = fixtures::unit::unit();

        let res = game.set_unit(
            fixtures::grid::x_in_grid(),
            fixtures::grid::y_in_grid(),
            unit.clone(),
        );
        assert!(res.is_ok());
        let hex = game
            .field
            .get_hex(fixtures::grid::x_in_grid(), fixtures::grid::y_in_grid())
            .unwrap();
        assert!(hex.unit.is_some());
        assert!(hex.content.is_none());

        let field_unit = hex.unit.as_ref().unwrap();
        assert_eq!(field_unit.player, fixtures::unit::player());
        assert_eq!(field_unit.hp, fixtures::unit::hp());
        assert_eq!(field_unit.attack, fixtures::unit::attack());
        assert_eq!(field_unit.speed, fixtures::unit::speed());

        let res = game.set_unit(
            fixtures::grid::x_out_grid(),
            fixtures::grid::y_out_grid(),
            unit,
        );
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "no hex");
    }

    #[test]
    fn set_content() {
        let mut game = Game::new(fixtures::grid::row_n(), fixtures::grid::col_n());

        let res = game.set_content(
            fixtures::grid::x_in_grid(),
            fixtures::grid::y_in_grid(),
            fixtures::content::content_wall(),
        );
        assert!(res.is_ok());
        let hex = game
            .field
            .get_hex(fixtures::grid::x_in_grid(), fixtures::grid::y_in_grid())
            .unwrap();
        assert!(hex.unit.is_none());
        assert!(hex.content.is_some());

        let res = game.set_content(
            fixtures::grid::x_out_grid(),
            fixtures::grid::y_out_grid(),
            fixtures::content::content_wall(),
        );
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "no hex");
    }

    #[test]
    fn serialize() {
        let game = Game::new(fixtures::grid::row_n(), fixtures::grid::col_n());

        let game_string = serde_json::to_string(&game).unwrap();
        assert_eq!(
            game_string,
            format!(
                "{{\"row_n\":{},\"col_n\":{},\"field\":{}}}",
                fixtures::grid::row_n(),
                fixtures::grid::col_n(),
                serde_json::to_string(&game.field).unwrap()
            ),
        );
    }
}
