///
/// Code for the stage 1 and stage 2 exercies for the day
/// 
use std::io;
use std::num::ParseIntError;

use crate::string::{find_of_many, rfind_of_many};

pub type InputData = Vec<String>;
pub type OutputData = usize;

const NUMBERS:[&str;20] = [
    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9",
    "0", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"
];

///
/// Compute the result for Stage 1
/// 
/// # Errors
///     May return an Error::ParseError if no digits are found
///
/// 
pub fn one(input: &InputData) -> Result<OutputData,Error> {
    Ok(input.iter().map(
        |line| {
            let mut digit_found = false;
            let mut first_digit = '\0';
            let mut last_digit = '\0';

            for c in line.chars() { 
                if c.is_ascii_digit() {
                    if !digit_found {
                        first_digit = c;
                        digit_found = true;
                    }
                    last_digit = c;
                }
            }
            format!("{first_digit}{last_digit}").parse::<usize>()
        }
    ).sum::<Result<usize,ParseIntError>>()?)
}

///
/// Compute the result for Stage 2
/// 
/// # Errors
///     May return an Errror if no digits are found
/// 
pub fn two(input: &InputData) -> Result<OutputData,Error> {
    let result = input.iter().map(
        |line| {
            let first_digit = if let Some(num) = find_of_many(line, &NUMBERS) {
                if num < 10 { num } else { num - 10 }
            } else {
                return Err(Error::ParseError)
            };
            let last_digit = if let Some(num) = rfind_of_many(line, &NUMBERS) {
                if num < 10 { num } else { num - 10 }
            } else {
                return Err(Error::ParseError)
            };
            Ok(first_digit * 10 + last_digit)
        }
    ).sum::<Result<usize,Error>>()?;
    Ok(result)
}

#[derive(Debug)]
pub enum Error {
    IO(io::Error),
    ParseError
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
