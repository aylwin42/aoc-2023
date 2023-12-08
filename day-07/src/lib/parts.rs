///
/// Code for the part 1 and part 2 exercies for the day 07
/// 

use crate::Error;
use crate::camel::Hand;

pub type InputData = Vec<(Hand,u32)>;
pub type OutputData = usize;

///
/// Compute the result for Stage 1
/// 
/// # Errors
///     May return an Error
/// 
pub fn one(input: &InputData) -> Result<OutputData,Error> {
    let mut input = (*input).clone();

    input.sort_by(|a, b| a.0.cmp(&b.0));

    Ok(
        input.iter().enumerate().map(|(rank,(_hand,bid))| {
            let rank = rank + 1;
            rank * *bid as usize
        }
        ).sum()
    )
}

///
/// Compute the result for Stage 2
/// 
/// # Errors
///     May return an Errror
/// 
pub fn two(input: &InputData) -> Result<OutputData,Error> {
    let mut input = input.iter()
        .map(|(hand,bid)| (hand.with_jokers(),*bid))
        .collect::<Vec<_>>();

    input.sort_by(|a, b| a.0.cmp(&b.0));

    Ok(
        input.iter().enumerate().map(|(rank,(_hand,bid))| {
            let rank = rank + 1;
            rank * *bid as usize
        }
        ).sum()
    )
}
