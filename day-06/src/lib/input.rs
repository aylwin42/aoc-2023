use std::fs::File;
use std::io;
use std::io::{BufRead,BufReader,Lines};
use std::path::Path;

use crate::parts::{InputData,Error};
use crate::boat::Race;

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
    let mut lines = read_lines(file_path)?;
    let times;
    let distances;

    if let Some(time_line) = lines.next() {
        let time_line = time_line?;
        if !time_line.starts_with("Time:") {
            return Err(Error::ParseError);
        }
        times = time_line["Time:".len()..].split_whitespace().map(str::parse::<usize>).collect::<Result<Vec<_>,_>>()?;
    } else {
        return Err(Error::ParseError)
    }

    if let Some(distance_line) = lines.next() {
        let distance_line = distance_line?;
        if !distance_line.starts_with("Distance:") {
            return Err(Error::ParseError)
        }
        distances = distance_line["Distance:".len()..].split_whitespace().map(str::parse::<usize>).collect::<Result<Vec<_>,_>>()?;
    } else {
        return Err(Error::ParseError)
    }

    if times.len() != distances.len() {
        return Err(Error::ParseError)
    }

    Ok(
        distances.iter().zip(times.iter()).map(
            |(distance, time)| {
                Race { best_time: *time, distance: *distance }
            }
        ).collect::<Vec<_>>()
    )
}