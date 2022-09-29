use array2d::Array2D;

mod game_of_life;

fn main() {

    let row: usize = 100;
    let col: usize = 100;

    let grid_a: Array2D<i32> = game_of_life::initialize_grid(game_of_life::read_file_to_string_buffer("input.txt"),game_of_life::make_grid(row,col));
    let grid_b: Array2D<i32>  = game_of_life::make_grid(row,col);

    game_of_life::write_to_file(game_of_life::game_of_life(grid_a,grid_b),"output.txt");
}
