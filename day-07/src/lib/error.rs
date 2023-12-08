use std::io;
use std::num::ParseIntError;

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