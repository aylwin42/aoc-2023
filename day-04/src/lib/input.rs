use std::fs::File;
use std::io;
use std::io::{BufRead,BufReader,Lines};
use std::path::Path;

use crate::stage::{InputData,Error};
use crate::scratchoff::Card;


// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<Lines<BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

///
/// Load and parse the input for Advent of Code 2023 Day 04's exercise
/// 
/// # Errors
///     returns io::Error if there is trouble opening the file
/// 
pub fn load<P>(file_path: P) -> Result<InputData,Error>
    where P: AsRef<Path>
{
    let lines = read_lines(file_path)?;

    lines.into_iter().map(
        |line| {
            let line = line?;
            let card = Card::parse(line.as_str())?;
            Ok(card)
        }
    ).collect::<Result<Vec<_>,Error>>()
}