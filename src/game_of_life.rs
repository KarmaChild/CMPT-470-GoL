/// **Conway's Game of Life Rules**
///
/// Conway's Game of Life operates on a grid where each cell can be alive (1) or dead (0). The game evolves through generations based on the following rules:
///
/// **Note**: In this implementation, the grid size is limited to a maximum of 100x100 cells.
/// 1. **Underpopulation**:
///    - Any live cell with fewer than two live neighbors dies, as if by underpopulation.
///
/// 2. **Survival**:
///    - Any live cell with two or three live neighbors survives to the next generation.
///
/// 3. **Overpopulation**:
///    - Any live cell with more than three live neighbors dies, as if by overpopulation.
///
/// 4. **Reproduction**:
///    - Any dead cell with exactly three live neighbors becomes a live cell, as if by reproduction.
///
/// These rules determine the state of each cell in the grid based on its current state and the number of live neighbors it has. The game progresses in discrete generations, with each generation determined by applying these rules simultaneously to every cell in the grid.
///
/// Understanding these rules is essential for simulating the dynamics of Conway's Game of Life, where complex patterns and behaviors can emerge from simple initial configurations.
///

use array2d::Array2D;



const NUMBER_OF_GENERATIONS: usize = 100;


/// Returns a 2d-Array grid with the specified size of type `Array2D<i8>`
///
/// # Arguments
///
/// * `rows` - A usize value > 0  that holds the number of rows in the 2d-Array grid
/// * `cols` - A usize value > 0 that holds the number of columns in the 2d-Array grid
///
/// # Preconditions
///
/// * `rows`==`cols` to keep the square shape
///
pub fn make_grid(rows: i8, cols: i8) -> Array2D<i8> {
    assert_eq!(rows, cols);
    assert!(rows > 0);
    assert!(cols > 0);

    let  grid = Array2D::filled_with(0, rows as usize, cols as usize);
    grid
}


///Reads a String containing a (initial) state of the grid and returns the state in a 2d-Array grid of type `Array2D<i8>`
///
/// # Arguments
///
/// * `initial_grid_state` - The (initial) grid state as type `String`
///* `grid` - The grid to be populated with the (initial) state as type `Array2D<i8>`
///
pub fn initialize_grid(initial_grid_state: String, mut grid: Array2D<i8>) -> Array2D<i8> {
    let mut pos: (usize, usize) = (0, 0);

    for i in initial_grid_state.chars() {
        if i == '*' {
            grid.set(pos.0, pos.1, 1).expect("Cannot set grid value");
            pos.1 += 1;
        } else if i == ' ' {
            pos.1 += 1;
        } else if i == '\n' {
            pos.1 = 0;
            pos.0 += 1;
        }
    }

    grid
}


/// Calculates and returns the neighboring cell of at a specific direction of a cell as a tuple of type `(usize,usize)`
///
/// # Arguments
///
/// * `cell_position` - The position of the current cell as a tuple of type `(i8,i8)`
/// * `move_direction` - The direction to which the neighbor has to be found as a tuple of type `(i8,i8)`
///
fn neighbor_position(cell_position: (i8,i8), move_direction: (i8,i8)) -> (usize,usize) {
    let neighbor_position = ((cell_position.0 + move_direction.0) as usize,(cell_position.1+move_direction.1) as usize);
    neighbor_position
}


/// Returns the number of alive neighbors of a cell as type `i8`
///
/// # Arguments
///
/// * `cell_position` - The position of the cell as a tuple of type `(i8,i8)` for which the number of alive cells has to be found
/// * `grid` - The grid where the cell given at `cell_position` resides as type `Array2D<i8>`
fn number_of_alive_neighbors(cell_position: (i8, i8), grid: &Array2D<i8>) -> i8 {
    let mut alive: i8 = 0;

    let directions: [(i8,i8); 8] = [(-1,0),(1,0),(0,1),(0,-1),(-1,1),(-1,-1),(1,1),(1,-1)];

    for direction in directions{
        let neighbor = grid.get(neighbor_position(cell_position,direction).0, neighbor_position(cell_position,direction).1);
        if neighbor == Some(&1){
            alive += 1
        }
        if neighbor == None {
            continue;
        }
    }

    alive
}


///Calculates if a cell will be dead or alive in the next generation by calculating the number of
/// alive cells in its neighborhood and sets up a grid with the updated cell values and returns the
/// new updated grid as type `Array2D<i8>`
///
/// # Arguments
///
/// * `iterator` - A tuple of type `(i8,i8)` used for iterating the grid
/// * `starting_grid` - The grid of type `Array2D<i8>` to be updated
/// * `final_grid` - The grid of type `Array2D<i8>` that will hold the newly updated cell values
fn set_grid_values(iterator: (i8, i8), starting_grid: &Array2D<i8>, final_grid: &mut Array2D<i8>) {
    let alive_neighbors = number_of_alive_neighbors(iterator, starting_grid);
    let current_cell = starting_grid.get(iterator.0 as usize, iterator.1 as usize).unwrap_or(&0);

    let new_value = match (alive_neighbors, current_cell) {
        (3, _) => 1,
        (2, 1) => 1,
        _ => 0,
    };

    final_grid.set(iterator.0 as usize, iterator.1 as usize, new_value).expect("Cannot set grid value");
}


///Returns the state of the grid after the number of given generations
///
/// # Arguments
///
/// * `grid_a` - A 2d-Array of type `Array2D<i8>` containing the initial state of the grid
/// * `grid_b` - A helper 2d-Array grid of type `Array2D<i8>` to store the state of every other generation
///
/// # preconditions
///
/// * size && geometry of `grid_a`== size && geometry of `grid_b`
///
pub fn game_of_life(mut grid_a: Array2D<i8>, mut grid_b: Array2D<i8>) -> Array2D<i8> {
    assert_eq!(grid_a.num_rows(), grid_b.num_rows());
    assert_eq!(grid_a.num_columns(), grid_b.num_columns());

    for generation in 0..NUMBER_OF_GENERATIONS {
        let (current_grid, next_grid) = if generation % 2 == 0 {
            (&grid_a, &mut grid_b)
        } else {
            (&grid_b, &mut grid_a)
        };

        for row in 0..current_grid.num_rows() {
            for col in 0..current_grid.num_columns() {
                set_grid_values((row as i8, col as i8), current_grid, next_grid);
            }
        }
    }

    if NUMBER_OF_GENERATIONS % 2 == 0 {
        grid_a
    } else {
        grid_b
    }
}