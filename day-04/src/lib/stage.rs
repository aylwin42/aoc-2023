///
/// Code for the stage 1 and stage 2 exercies for the day
/// 
use std::{io, num::ParseIntError};

use crate::scratchoff::Card;

pub type InputData = Vec<Card>;
pub type OutputData = u32;

///
/// Compute the result for Stage 1
/// 
/// # Errors
///     May return an Error
/// 
pub fn one(input: &InputData) -> Result<OutputData,Error> {
    Ok(input.iter().map(
        Card::score
    ).sum())
}

///
/// Compute the result for Stage 2
/// 
/// # Errors
///     May return an Errror
/// 
pub fn two(input: &InputData) -> Result<OutputData,Error> {
    let mut card_data = input.iter().map(|_| 1).collect::<Vec<u32>>();

    for (idx, card) in input.iter().enumerate() {
        let matches = card.winning_num().len();
        let num_cards = card_data[idx];

        for idx in idx..idx+matches {
            card_data[idx + 1] += num_cards;
        }
    }

    Ok(card_data.iter().sum())
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