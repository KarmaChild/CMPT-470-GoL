use array2d::Array2D;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufRead, BufReader};
use std::any::type_name;

///Returns the type of a variable as a String
///
/// # Arguments
///
/// * `T` - A variable
fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

/// Returns a 2d-Array grid with the specified size of type `Array2D<i32>`
///
/// # Arguments
///
/// * `rows` - A usize value > 0  that holds the number of rows in the 2d-Array grid
/// * `cols` - A usize value > 0 that holds the number of columns in the 2d-Array grid
///
///
pub fn make_grid(rows:usize,cols:usize) -> Array2D<i32> {

    assert_eq!(type_of(rows),"usize");
    assert_eq!(type_of(cols),"usize");

    let  grid = Array2D::filled_with(0,rows,cols);
    return grid;
}

/// Reads the input file and returns the initial state of the grid as a 2d-Array grid of type `Array2D<i32>`
///
/// # Arguments
///
/// * `filename` - The name of the input file as a `&str` type
///
pub fn read_file_to_string_buffer(filename: &str) -> String{

    assert_eq!(type_of(filename),"&str");

    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);

    let mut initial_grid_state = String::new();
    while let Ok(n) = reader.read_line(&mut initial_grid_state) {
        if n == 0 { break; } // eof
    }
    return initial_grid_state;
}

/// Reads a 2d-Array grid and writes the grid state to a file
///
/// # Arguments
///
/// * `grid` - A grid of type `Array2D<i32>` to be written to a file
/// * `filename` - The name of the output file as type `&str`
///
/// # Postconditions
///
/// * this function does not return anything
///
pub fn write_to_file(grid: Array2D<i32>,filename: &str)->(){

    assert_eq!(type_of(grid.clone()),"array2d::Array2D<i32>");
    assert_eq!(type_of(filename),"&str");

    let mut output_file = File::create(filename).expect("Unable to create file");
    let mut line: i32 = 0;
    let mut output_string = String::new();
    for i in grid.elements_row_major_iter(){

        if line>0 && line%grid.num_rows() as i32 ==0 {
            output_string.push('\n');
        }

        if i==&1{
            output_string.push('*');
        }
        else if i == &0{
            output_string.push(' ');
        }
        line = line+1;
    }
    output_file.write_all(output_string.as_bytes()).expect("Unable to write to file");
}

///Reads a String containing a (initial) state of the grid and returns the state in a wd-Array grid of type `Array2D<i32>`
///
/// # Arguments
///
/// * `initial_grid_state` - The (initial) grid state as type `String`
///* `grid` - The grid to be populated with the (initial) state as type `Array2D<i32>`
///
pub fn initialize_grid(initial_grid_state:String,mut grid:Array2D<i32>) -> Array2D<i32>{

    assert_eq!(type_of(initial_grid_state.clone()),"alloc::string::String");
    assert_eq!(type_of(grid.clone()),"array2d::Array2D<i32>");

    let mut pos:  (usize, usize) =  (0, 0);
    for i in initial_grid_state.chars(){
        if i=='*' {
            grid.set(pos.0,pos.1, 1).expect("Cannot set grid value");
            pos.1=pos.1+1;
        }
        else if i==' ' {
            pos.1=pos.1+1;
        }
        else if i=='\n' {
            pos.1=0;
            pos.0=pos.0+1;
        }
    }
    return grid;
}


/// Calculates and returns the neighboring cell of at a specific direction of a cell as a tuple of type `(usize,usize)`
///
/// # Arguments
///
/// * `cell_position` - The position of the current cell as a tuple of type `(i32,i32)`
/// * `move_direction` - The direction to which the neighbor has to be found as a tuple of type `(i32,i32)`
///
fn neighbor_position(cell_position: (i32,i32), move_direction: (i32,i32)) -> (usize,usize){

    assert_eq!(type_of(cell_position),"(i32, i32)");
    assert_eq!(type_of(move_direction),"(i32, i32)");

    let neighbor_position: (usize,usize) = ((cell_position.0+move_direction.0) as usize,(cell_position.1+move_direction.1) as usize);
    return neighbor_position;
}

/// Returns the number of alive neighbors of a cell as type `i32`
///
/// # Arguments
///
/// * `cell_position` - The position of the cell as a tuple of type `(i32,i32)` for which the number of alive cells has to be found
/// * `grid` - The grid where the cell given at `cell_position` resides as type `Array2D<i32>`
fn number_of_alive_neighbors(cell_position: (i32,i32), grid:Array2D<i32>) -> i32{

    assert_eq!(type_of(cell_position),"(i32, i32)");
    assert_eq!(type_of(grid.clone()),"array2d::Array2D<i32>");

    let mut alive:i32 = 0;

    let directions: [(i32,i32);8] = [(-1,0),(1,0),(0,1),(0,-1),(-1,1),(-1,-1),(1,1),(1,-1)];

    for direction in directions{
        let neighbor = grid.get(neighbor_position(cell_position,direction).0,neighbor_position(cell_position,direction).1);
        if neighbor == Some(&1){
            alive=alive+1
        }
        if neighbor == None {
            continue;
        }
    }
    return alive;
}


///Calculates if a cell will be dead or alive in the next generation by calculating the number of
/// alive cells in its neighborhood and sets up a grid with the updated cell values and returns the
/// new updated grid as type `Array2D<i32>`
///
/// # Arguments
///
/// * `iterator` - A tuple of type `(i32,i32)` used for iterating the grid
/// * `starting_grid` - The grid of type `Array2D<i32>` to be updated
/// * `final_grid` - The grid of type `Array2D<i32>` that will hold the newly updated cell values
fn set_grid_values(iterator:(i32,i32), starting_grid:Array2D<i32>, mut final_grid:Array2D<i32>) -> Array2D<i32>{

    assert_eq!(type_of(iterator),"(i32, i32)");
    assert_eq!(type_of(starting_grid.clone()),"array2d::Array2D<i32>");
    assert_eq!(type_of(final_grid.clone()),"array2d::Array2D<i32>");

    if number_of_alive_neighbors(iterator,starting_grid.clone())==3 {
        final_grid.set(iterator.0.try_into().unwrap(),iterator.1.try_into().unwrap(),1).expect("Cannot set grid value");
    }
    if number_of_alive_neighbors(iterator,starting_grid.clone())==2 && starting_grid.get(iterator.0.try_into().unwrap(),iterator.1.try_into().unwrap())==Some(&1) {
        final_grid.set(iterator.0.try_into().unwrap(),iterator.1.try_into().unwrap(),1).expect("Cannot set grid value");
    }
    if number_of_alive_neighbors(iterator,starting_grid.clone())==2 && starting_grid.get(iterator.0.try_into().unwrap(),iterator.1.try_into().unwrap())==Some(&0) {
        final_grid.set(iterator.0.try_into().unwrap(),iterator.1.try_into().unwrap(),0).expect("Cannot set grid value");
    }
    if number_of_alive_neighbors(iterator,starting_grid.clone())>3 || number_of_alive_neighbors(iterator,starting_grid.clone())<2 {
        final_grid.set(iterator.0.try_into().unwrap(),iterator.1.try_into().unwrap(),0).expect("Cannot set grid value");
    }

    return final_grid;
}


///Returns the state of the grid after 100 generations
///
/// # Arguments
///
/// * `grid_a` - A 2d-Array of type `Array2D<i32>` containing the initial state of the grid
/// * `grid_b` - A helper 2d-Array grid of type `Array2D<i32>` to store the state of every other generation
///
/// # preconditions
///
/// * size && geometry of `grid_a`== size && geometry of `grid_b`
///
pub fn game_of_life(mut grid_a:Array2D<i32>,mut grid_b:Array2D<i32>) -> Array2D<i32>{

    assert_eq!(type_of(grid_a.clone()),"array2d::Array2D<i32>");
    assert_eq!(type_of(grid_b.clone()),"array2d::Array2D<i32>");
    assert_eq!(grid_a.num_rows(),grid_b.num_rows());
    assert_eq!(grid_a.num_columns(),grid_b.num_columns());

    for generation in 0..100{
        let mut grid_b_iter:(i32,i32) = (0,0);
        let mut grid_a_iter:(i32,i32) = (0,0);

        if generation%2==0{
            for _row in grid_a.as_rows(){
                for _col in _row{
                    grid_b=set_grid_values(grid_a_iter,grid_a.clone(),grid_b.clone());
                    grid_a_iter.1=grid_a_iter.1+1;
                }
                grid_a_iter.0=grid_a_iter.0+1;
                grid_a_iter.1=0;
            }

        }
        else {
            for _row in grid_b.as_rows(){
                for _col in _row{
                    grid_a=set_grid_values(grid_b_iter,grid_b.clone(),grid_a.clone());
                    grid_b_iter.1=grid_b_iter.1+1;
                }
                grid_b_iter.0=grid_b_iter.0+1;
                grid_b_iter.1=0;
            }
        }
    }
    return grid_a;
}