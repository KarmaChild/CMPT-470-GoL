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

    fn move_to_neighbor(cell_position: (i32,i32), move_direction: (i32,i32)) -> (usize,usize){
        let neighbor_position: (usize,usize) = ((cell_position.0+move_direction.0) as usize,(cell_position.1+move_direction.1) as usize);
        return neighbor_position;
    }

    fn number_of_alive_neighbors(cell_position: (i32,i32), grid:Array2D<i32>) -> i32{
        let mut alive:i32 = 0;

        let directions: [(i32,i32);8] = [(-1,0),(1,0),(0,1),(0,-1),(-1,1),(-1,-1),(1,1),(1,-1)];

        for direction in directions{
            let neighbor = grid.get(move_to_neighbor(cell_position,direction).0,move_to_neighbor(cell_position,direction).1);
            if neighbor == Some(&1){
                alive=alive+1
            }
            if neighbor == None {
                continue;
            }
        }
        return alive;
    }


    fn game_of_life(mut grid_a:Array2D<i32>,mut grid_b:Array2D<i32>) -> Array2D<i32>{
        for generation in 0..1{
            let mut grid_b_iter:(i32,i32) = (0,0);
            let mut grid_a_iter:(i32,i32) = (0,0);

            if generation%2!=0{
                for row in grid_b.as_rows(){
                    for col in row{
                        if number_of_alive_neighbors(grid_b_iter,grid_b.clone())==3{
                            grid_a.set(grid_b_iter.0.try_into().unwrap(),grid_b_iter.1.try_into().unwrap(),1);
                        }
                        if number_of_alive_neighbors(grid_b_iter,grid_b.clone())==2 && grid_b.get(grid_b_iter.0.try_into().unwrap(),grid_b_iter.1.try_into().unwrap())==Some(&1) {
                            grid_a.set(grid_b_iter.0.try_into().unwrap(),grid_b_iter.1.try_into().unwrap(),1);
                        }
                        else {
                            grid_a.set(grid_b_iter.0.try_into().unwrap(),grid_b_iter.1.try_into().unwrap(),0);
                        }
                        grid_b_iter.1=grid_b_iter.1+1;
                    }
                    grid_b_iter.0=grid_b_iter.0+1;
                    grid_b_iter.1=0;
                }
            }
            else {
                for row in grid_a.as_rows(){
                    for col in row{
                        if number_of_alive_neighbors(grid_a_iter,grid_a.clone())==3{
                            grid_b.set(grid_a_iter.0.try_into().unwrap(),grid_a_iter.1.try_into().unwrap(),1);
                        }
                        if number_of_alive_neighbors(grid_a_iter,grid_a.clone())==2 && grid_a.get(grid_a_iter.0.try_into().unwrap(),grid_a_iter.1.try_into().unwrap())==Some(&1) {
                            grid_b.set(grid_a_iter.0.try_into().unwrap(),grid_a_iter.1.try_into().unwrap(),1);
                        }
                        else {
                            grid_b.set(grid_a_iter.0.try_into().unwrap(),grid_a_iter.1.try_into().unwrap(),0);
                        }
                        grid_a_iter.1=grid_a_iter.1+1;
                    }
                    grid_a_iter.0=grid_a_iter.0+1;
                    grid_a_iter.1=0;
                }
            }
        }
        return grid_b;
    }

    let mut a = initialize_grid(read_file_to_string_buffer("input.txt"),make_grid(100,100));
    let mut b = make_grid(100,100);

    //println!("{:?}",game_of_life(a,b));

    write_to_file(game_of_life(a,b),"output.txt");

    fn write_to_file(grid: Array2D<i32>,filename: &str)->(){
        let mut output_file = File::create(filename).expect("Unable to create file");
        let mut line: i32 = 0;
        let mut output_string = String::new();
        for i in grid.elements_row_major_iter(){

            if line>0 && line%100==0 {
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
}
