use std::{io, process};
use std::io::Write;
use array2d::Array2D;
mod game_of_life;

fn main() {

    println!("For a square grid, give the same number of rows and columns");
    println!("For a rectangular grid, make either number of rows or columns larger than the other");
    println!("Enter the Number of rows:");

    io::stdout().flush().unwrap();
    let mut row_input = String::new();
    io::stdin().read_line(&mut row_input).unwrap();

    let rows = row_input.trim().parse::<usize>().unwrap_or_else(|_| {
        eprintln!("- Entered input is not an Integer!");
        drop(row_input);
        process::exit(1);
    });

    println!("Enter the Number of columns:");

    let mut col_input = String::new();
    io::stdin().read_line(&mut col_input).unwrap();

    let cols = col_input.trim().parse::<usize>().unwrap_or_else(|_| {
        eprintln!("- Entered input is not an Integer!");
        drop(col_input);
        process::exit(1);
    });

    println!("Processing...");
    let grid_a: Array2D<i32> = game_of_life::initialize_grid(game_of_life::read_file_to_string_buffer("input.txt"),game_of_life::make_grid(rows,cols));
    let grid_b: Array2D<i32>  = game_of_life::make_grid(rows,cols);

    game_of_life::write_to_file(game_of_life::game_of_life(grid_a,grid_b),"output.txt");

    println!("Check the output file!")
}
