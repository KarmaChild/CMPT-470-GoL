use std::fs::File;
use std::io::{BufRead, BufReader, Error, Write};
use array2d::Array2D;

/// Reads the input file and returns the initial state of the grid as a 2d-Array grid of type `Array2D<i32>`
///
/// # Arguments
///
/// * `filename` - The name of the input file as a `&str` type
///
pub fn read_file_to_string_buffer(filename: &str) -> Result<String, Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    // Read the file contents into a string buffer
    let mut initial_grid_state = String::new();
    for line in reader.lines() {
        initial_grid_state.push_str(&line?);
        initial_grid_state.push('\n');
    }

    assert!(!initial_grid_state.is_empty(), "The file is empty");

    Ok(initial_grid_state)
}

/// Reads a 2d-Array grid and writes the grid state to a file
///
/// # Arguments
///
/// * `grid` - A grid of type `Array2D<i32>` to be written to a file
/// * `filename` - The name of the output file as type `&str`
///
pub fn write_to_file(grid: &Array2D<i8>, filename: &str) -> Result<(), Error> {
    let mut output_file = File::create(filename)?;

    let mut line: usize = 0;
    let mut output_string = String::new();

    for (index, &value) in grid.elements_row_major_iter().enumerate() {
        if line > 0 && line % grid.num_rows() == 0 {
            output_string.push('\n');
        }

        if value == 1 {
            output_string.push('*');
        } else {
            output_string.push(' ');
        }

        line += 1;

        // Flush every 100 characters to avoid buffering issues
        if index > 0 && index % 100 == 0 {
            output_file.write_all(output_string.as_bytes())?;
            output_string.clear();
        }
    }

    // Write any remaining characters to file
    if !output_string.is_empty() {
        output_file.write_all(output_string.as_bytes())?;
    }

    Ok(())
}