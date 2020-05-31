use super::game_objects::grid::Grid;
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
}

#[cfg(test)]
mod test {
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
}
