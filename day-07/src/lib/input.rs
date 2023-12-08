use std::fs::File;
use std::io;
use std::io::{BufReader,BufRead,Lines};
use std::path::Path;
use std::str::FromStr;

use crate::Error;
use crate::camel::Hand;
use crate::parts::InputData;


// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<Lines<BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
///
/// Load and parse the input for Advent of Code 2023 Day's exercise
/// 
/// # Errors
///     returns io::Error if there is trouble opening the file
/// 
pub fn load<P>(file_path: P) -> Result<InputData,Error>
    where P: AsRef<Path>
{
    read_lines(file_path)?.map(|line| {
        let line = line?;
        let (hand_str,bid_str) = line.split_once(' ').ok_or(Error::ParseError)?;

        Ok((Hand::from_str(hand_str)?, bid_str.parse::<u32>()?))
    }).collect()
}