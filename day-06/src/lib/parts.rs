///
/// Code for the stage 1 and stage 2 exercies for the day
/// 
use std::{io, num::ParseIntError};

use crate::boat::Race;

pub type InputData = Vec<Race>;
pub type OutputData = usize;

///
/// Compute the result for Stage 1
/// 
/// # Errors
///     May return an Error
/// 
pub fn one(input: &InputData) -> Result<OutputData,Error> {
    dbg!(input);
    Ok(input.iter().map(Race::num_winners).product())
}

///
/// Compute the result for Stage 2
/// 
/// # Errors
///     May return an Errror
/// 
pub fn two(input: &InputData) -> Result<OutputData,Error> {
    let (best_time_str, distance_str) = input.iter().fold((String::new(),String::new()), |(time, distance), race| {
        (format!("{}{}", time, race.best_time), format!("{}{}", distance, race.distance))
    });

    let best_time = best_time_str.parse::<usize>()?;
    let distance =distance_str.parse::<usize>()?;

    Ok(Race{best_time, distance}.num_winners())

}

#[derive(Debug)]
pub enum Error {
    IO(io::Error),
    ParseError,
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
