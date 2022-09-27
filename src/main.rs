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

}
