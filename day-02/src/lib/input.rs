use std::fs::File;
use std::io::Read;
use std::path::Path;

use crate::stage::{Game,Sample,InputData,Error};

fn parse_sample(sample_txt: &str) -> Sample {
    // Samples look like -
    // 3 blue, 4 red
    // 1 red, 2 green, 6 blue
    // 2 green

    let mut sample = Sample::default();

    for sample_part in sample_txt.split(',') {
        let sample_part = sample_part.trim();
        let parts = sample_part.split_whitespace().collect::<Vec<_>>();

        let num_cubes = parts[0].parse::<u32>().unwrap();
        match parts[1] {
            "red" => sample.red = num_cubes,
            "green" => sample.green = num_cubes,
            "blue" => sample.blue = num_cubes,
            _ => panic!("Unknown cube color")
        }
    }
    sample
}

fn parse_game(line: &str) -> Game {

    // First split at : to get game and samples
    let parts = line.split(':').collect::<Vec<_>>();

    let game_id = parts[0][5..].parse::<u32>().unwrap();
    let samples = parts[1].split(';').map(parse_sample).collect::<Vec<_>>();

    Game {id: game_id, samples}
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
    let mut file = File::open(file_path)?;
    let mut buf = String::new();

    file.read_to_string(&mut buf)?;

    Ok(buf.lines().map(parse_game).collect::<Vec<_>>())
}