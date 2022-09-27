#[allow(unused_variables)]
use array2d::Array2D;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufRead, BufReader};

fn main() {

    fn make_grid(rows:usize,cols:usize) -> Array2D<i32> {
        let mut grid = Array2D::filled_with(0,rows,cols);
        return grid;
    }

    fn read_file_to_string_buffer(filename: &str) -> String{
        let file = File::open(filename).unwrap();
        let mut reader = BufReader::new(file);

        let mut initial_grid_state = String::new();
        while let Ok(n) = reader.read_line(&mut initial_grid_state) {
            if n == 0 { break; } // eof
            //buf.clear(); // otherwise the data will accumulate in your buffer
        }
        return initial_grid_state;
    }

    fn initialize_grid(initial_grid_state:String,mut grid:Array2D<i32>) -> Array2D<i32>{
        let mut pos:  (usize, usize) =  (0, 0);
        for i in initial_grid_state.chars(){
            if i=='*' {
                grid.set(pos.0,pos.1, 1);
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

    println!("{:?}",initialize_grid(read_file_to_string_buffer("input.txt"),make_grid(10,10)));

    fn move_to_neighbor(cell_position: (i32,i32), move_direction: (i32,i32)) -> (usize,usize){
        let neighbor_position: (usize,usize) = ((cell_position.0+move_direction.0) as usize,(cell_position.1+move_direction.1) as usize);
        return neighbor_position;
    }

    fn number_of_alive_neighbors(cell_position: (i32,i32), grid:Array2D<i32>) -> i32{
        let mut alive:i32 = 0;

        let directions: [(i32,i32);8] = [(-1,0),(1,0),(0,1),(0,-1),(-1,1),(-1,-1),(1,1),(1,-1)];

        for i in directions{
            let neighbor = grid.get(move_to_neighbor(cell_position,i).0,move_to_neighbor(cell_position,i).1);
            if neighbor == Some(&1){
                alive=alive+1
            }
            if neighbor == None {
                continue;
            }
        }
        return alive;
    }

}
