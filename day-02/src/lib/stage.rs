///
/// Code for the stage 1 and stage 2 exercies for the day
/// 
use std::io;

#[derive(Debug,Default,Copy,Clone)]
pub struct Sample {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

impl Sample {
    fn possible(&self, bag: &Sample) -> bool {
        self.red <= bag.red && self.green <= bag.green && self.blue <= bag.blue
    }

    fn min(&self, other: &Sample) -> Sample {
        Sample {
            red: self.red.max(other.red),
            green: self.green.max(other.green),
            blue: self.blue.max(other.blue)
        }
    }

    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

pub struct Game {
    pub id: u32,
    pub samples: Vec<Sample>
}

impl Game {
    fn possible(&self, bag: &Sample) -> bool {
        self.samples.iter().all(|sample| sample.possible(bag))
    }

    fn min(&self) -> Sample {
        self.samples.iter().fold(Sample::default(),|bag, sample| bag.min(sample))
    }
}

pub type InputData = Vec<Game>;
pub type OutputData = u32;

///
/// Compute the result for Stage 1
/// 
/// # Errors
///     May return an Error
/// 
pub fn one(input: &InputData) -> Result<OutputData,Error> {
    let bag = Sample { red: 12, green: 13, blue: 14 };

    Ok(
        input.iter().filter_map(
            |game| {
                if game.possible(&bag) {
                    Some(game.id)
                } else {
                    None
                }
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
    Ok(
        input.iter().map(
            |game| {
                game.min().power()
            }
        ).sum()
    )
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
