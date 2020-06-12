use crate::game_objects::grid::Grid;
use rstest::*;

#[fixture]
pub fn row_n() -> u32 {
    5
}

#[fixture]
pub fn col_n() -> u32 {
    4
}

#[fixture]
pub fn grid() -> Grid {
    Grid::new(row_n(), col_n())
}

#[fixture()]
pub fn x_in_grid() -> u32 {
    row_n() - 1
}

#[fixture()]
pub fn x_out_grid() -> u32 {
    row_n() + 1
}

#[fixture()]
pub fn y_in_grid() -> u32 {
    col_n() - 1
}

#[fixture()]
pub fn y_out_grid() -> u32 {
    col_n() + 1
}
