///
/// Code for the stage 1 and stage 2 exercies for the day
/// 
use std::{io, collections::{HashMap, HashSet}};

pub type InputData = Vec<Vec<char>>;
pub type OutputData = u32;

fn is_symbol(item: char) -> bool {
    item != '.' && item.is_ascii_punctuation()
}

fn is_gear(item: char) -> bool {
    item == '*'
}

fn search_symbol(matrix: &[Vec<char>], x: usize, y: usize) -> bool {
    // Search the 8 cells around (x,y) for a symbol
    (x != 0 && y != 0 && is_symbol(matrix[y-1][x-1])) ||
    (y != 0 && is_symbol(matrix[y-1][x])) ||
    (y != 0 && x + 1 != matrix[y-1].len() && is_symbol(matrix[y-1][x+1])) ||
    (x != 0 && is_symbol(matrix[y][x-1])) ||
    (x + 1 != matrix[y].len() && is_symbol(matrix[y][x+1])) ||
    (x != 0 && y + 1 != matrix.len() && is_symbol(matrix[y+1][x-1])) ||
    (y + 1 != matrix.len() && is_symbol(matrix[y+1][x])) ||
    (y + 1 != matrix.len() && x + 1 != matrix[y+1].len() && is_symbol(matrix[y+1][x+1]))
}

// Find all the gears around a location
fn find_gears(matrix: &[Vec<char>], x: usize, y: usize) -> HashSet<(usize, usize)> {
    // Search the 8 cells around (x,y) for a symbol
    let mut result = HashSet::new();

    if x != 0 && y != 0 && is_gear(matrix[y-1][x-1]) {
        result.insert((x-1, y-1));
    }
    

    if y != 0 && is_gear(matrix[y-1][x]) {
        result.insert((x, y - 1));
    }

    if y != 0 && x + 1 != matrix[y-1].len() && is_symbol(matrix[y-1][x+1]) {
        result.insert((x + 1, y -1 ));
    }

    if x != 0 && is_gear(matrix[y][x-1]) {
        result.insert((x - 1, y));
    }

    if x + 1 != matrix[y].len() && is_gear(matrix[y][x+1]) {
        result.insert((x + 1, y));
    }

    if x != 0 && y + 1 != matrix.len() && is_gear(matrix[y+1][x-1]) {
        result.insert((x - 1, y + 1));
    }

    if y + 1 != matrix.len() && is_gear(matrix[y+1][x]) {
        result.insert((x, y + 1));
    }

    if y + 1 != matrix.len() && x + 1 != matrix[y+1].len() && is_gear(matrix[y+1][x+1]) {
        result.insert((x + 1, y + 1));
    }

    result
}


///
/// Compute the result for Stage 1
/// 
/// # Errors
///     May return an Error
/// 
pub fn one(input: &InputData) -> Result<OutputData,Error> {
    let mut result = 0;
    
    for (row_idx, row) in input.iter().enumerate() {
        let mut num = 0;
        let mut is_part = false;
        for (col_idx, item) in row.iter().enumerate() {
            if item.is_ascii_digit() {
                num = num * 10 + (*item as u32) - ('0' as u32);
                if !is_part {
                    is_part = search_symbol(input, col_idx, row_idx);
                }
                if col_idx < row.len() - 1 {
                    // If not at the end of the line, skip the next part and process
                    // the next character
                    continue;
                }
            }
            // If not a digit, or at the end of the line...
            if is_part {
                result += num;
            }
            num = 0;
            is_part = false;
        }
    }
    Ok(result)
}

///
/// Compute the result for Stage 2
/// 
/// # Errors
///     May return an Errror
/// 
pub fn two(input: &InputData) -> Result<OutputData,Error> {
    // List of possible gears with a list of the numbers found next to them
    let mut gears: HashMap<(usize,usize), Vec<u32>> = HashMap::new();

    for (row_idx, row) in input.iter().enumerate() {
        let mut gear_list = HashSet::new();
        let mut num = 0;

        // Find the number in the line and all the gears it is near
        // Then for each gear, add the number to its list for later analysis
        for (col_idx, item) in row.iter().enumerate() {
            if item.is_ascii_digit() {
                // Add the digit to the number
                num = num * 10 + (*item as u32) - ('0' as u32);

                // Find all the possible gears around the digit and add them to the gear_list set
                for coord in find_gears(input, col_idx, row_idx).drain() {
                    gear_list.insert(coord);
                }
                if col_idx < row.len() - 1 {
                    // Skip the next part if not at the end of the line and process the next character
                    continue;
                }
            }
            // if gear_list contains anything then we are at the symbol or end of line
            // after the end of a number, so add the number to the entry for that
            // (possible) gear
            for coord in gear_list.drain() {
                if let Some(list) = gears.get_mut(&coord) {
                    list.push(num);
                } else {
                    gears.insert(coord, vec![num]);
                }
            }
            num = 0;
        }
   }

    // Find all the gears that have two, and only two numbers
    // near it and compute the gear ratio, then sum the results
    Ok(gears.iter().filter_map(|(_,num_list)| {
        if num_list.len() == 2 {
            Some(num_list[0] * num_list[1])
        } else {
            None
        }
    }).sum())
}

#[derive(Debug)]
pub enum Error {
    IO(io::Error)
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::IO(value)
    }
}
