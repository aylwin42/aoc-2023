use std::fs::File;
use std::io::Read;
use std::path::Path;

use std::string::ToString;

use crate::stage::{InputData,Error};


///
/// Load and parse the input for Advent of Code 2023 Day's exercise
/// 
/// # Errors
///     returns io::Error if there is trouble opening the file
/// 
pub fn load<P>(file_path: P) -> Result<InputData,Error>
    where P: AsRef<Path>
{
    let mut file = File::open(file_path)?;
    let mut buf = String::new();
    
    file.read_to_string(&mut buf)?;

    Ok(buf.lines().map(ToString::to_string).collect::<Vec<_>>())
}