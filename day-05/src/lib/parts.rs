///
/// Code for the stage 1 and stage 2 exercies for the day
/// 
use std::io;
use std::num::ParseIntError;

use crate::almanac::{Seeds,Maps};

pub type InputData = (Seeds,Maps);
pub type OutputData = u64;

///
/// Compute the result for Stage 1
/// 
/// # Errors
///     May return an Error
/// 
/// # Panics
///    Panics if it cannot find a location for a seed
/// 
pub fn one(input: &InputData) -> Result<OutputData,Error> {
    // Find the closest location for any of the seeds

    let (seeds, map) = input;

    Ok(seeds.iter().map(|seed|
        map.seed_to_location(*seed)
    ).min().unwrap())
}

///
/// Compute the result for Stage 2
/// 
/// # Errors
///     May return an Errror
/// 
/// # Panics
///  May panic if there are no seeds
/// 
pub fn two(input: &InputData) -> Result<OutputData,Error> {
    let (seed_data, map) = input;

    let seed_ranges = (0..seed_data.len()).step_by(2).map(
        |idx| seed_data[idx]..(seed_data[idx]+seed_data[idx + 1])
    ).collect::<Vec<_>>();

    Ok(
        seed_ranges.iter().map(
            |seed_range| seed_range.clone().map(
                |seed| map.seed_to_location(seed)
            ).min().unwrap()
        ).min().unwrap()
    )
}

#[derive(Debug)]
pub enum Error {
    IO(io::Error),
    ParseError,
    LocationNotFound
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::IO(value)
    }
}

impl From<ParseIntError> for Error {
    fn from(_value: ParseIntError) -> Self {
        Self::ParseError
    }
}