use array2d::Array2D;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufRead, BufReader};

pub fn make_grid(rows:usize,cols:usize) -> Array2D<i32> {
    let  grid = Array2D::filled_with(0,rows,cols);
    return grid;
}

pub fn read_file_to_string_buffer(filename: &str) -> String{
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);

    let mut initial_grid_state = String::new();
    while let Ok(n) = reader.read_line(&mut initial_grid_state) {
        if n == 0 { break; } // eof
    }
    return initial_grid_state;
}

pub fn write_to_file(grid: Array2D<i32>,filename: &str)->(){
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

pub fn initialize_grid(initial_grid_state:String,mut grid:Array2D<i32>) -> Array2D<i32>{
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

fn neighbor_position(cell_position: (i32,i32), move_direction: (i32,i32)) -> (usize,usize){
    let neighbor_position: (usize,usize) = ((cell_position.0+move_direction.0) as usize,(cell_position.1+move_direction.1) as usize);
    return neighbor_position;
}

fn number_of_alive_neighbors(cell_position: (i32,i32), grid:Array2D<i32>) -> i32{
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

fn set_grid_values(iterator:(i32,i32), starting_grid:Array2D<i32>, mut final_grid:Array2D<i32>) -> Array2D<i32>{
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


pub fn game_of_life(mut grid_a:Array2D<i32>,mut grid_b:Array2D<i32>) -> Array2D<i32>{
    let number_of_gens:i32 = 20;
    for generation in 0..number_of_gens{
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
    if number_of_gens % 2 == 0 {
        return grid_a;
    }
    return grid_b;

}