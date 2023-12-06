use std::ops::Deref;
use std::num::ParseIntError;

/// Scratch Off Cards
use crate::Error;

#[derive(Debug)]
pub struct Card {
    pub card_id: u32,
    pub winning_num: Vec<u8>,
    pub scoring_num: Vec<u8>
}

impl Card {
    #[must_use]
    pub fn winning_num(&self) -> Vec<u8> {
        self.scoring_num.iter().copied()
            .filter(|num| self.winning_num.contains(num)).collect()
    }

    #[must_use]
    pub fn score(&self) -> u32 {
        (1<<self.winning_num().len())>>1
    }

    ///
    /// # Errors
    ///     Returns a ParseError if the input does not match the card specification
    /// 
    pub fn parse<P>(card_text: P) -> Result<Card,Error>
    where P: Deref<Target=str> {
        if !card_text.starts_with("Card") {
            return Err(Error::ParseError)
        }
        let parts = card_text.split(':').collect::<Vec<_>>();

        if parts.len() != 2 {
            return Err(Error::ParseError)
        }

        let id_parts = parts[0].split_whitespace().collect::<Vec<_>>();
        if id_parts.len() != 2 {
            return Err(Error::ParseError)
        }

        let card_id = id_parts[1].parse::<u32>()?;

        let parts = parts[1].split('|').collect::<Vec<_>>();

        if parts.len() != 2 {
            return Err(Error::ParseError)
        }

        let winning_num = parts[0].split_whitespace().map(
            str::parse
        ).collect::<Result<Vec<_>,_>>()?;
     
        let scoring_num = parts[1].split_whitespace().map(
            str::parse
        ).collect::<Result<Vec<_>,ParseIntError>>()?;
        
        Ok(Card { card_id, winning_num, scoring_num })
    }
}