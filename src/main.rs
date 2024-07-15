use array2d::Array2D;
mod game_of_life;
mod file_io;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let row: i8 = 127;
    let col: i8 = 127;

    let grid_state = file_io::read_file_to_string_buffer("input.txt")?;

    let grid_a: Array2D<i8> = game_of_life::initialize_grid(grid_state, game_of_life::make_grid(row, col));

    let grid_b: Array2D<i8> = game_of_life::make_grid(row, col);

    let final_grid: Array2D<i8> = game_of_life::game_of_life(grid_a, grid_b);

    file_io::write_to_file(&final_grid, "output.txt").expect("TODO: panic message");

    Ok(())
}
