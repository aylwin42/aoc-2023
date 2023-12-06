use std::fs::File;
use std::io;
use std::io::{BufRead,BufReader,Lines};
use std::path::Path;

use crate::almanac::{Map, MapEntry, Maps};
use crate::parts::{InputData,Error};

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<Lines<BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

fn load_map(lines: &mut Lines<BufReader<File>>, map_name: &str) -> Result<Map,Error> {
    if let Some(name) = lines.next() {
        let name = name?;
        if name != map_name {
            return Err(Error::ParseError)
        }
    } else {
        return Err(Error::ParseError)
    }

    let mut map = Map {
        name: map_name.to_string(),
        entries: Vec::new()
    };

    for line in lines.by_ref() {
        let line = line?;
        if line.is_empty() {
            break;
        }
        let nums = line.split_whitespace().map(str::parse::<u64>).collect::<Result<Vec<_>,_>>()?;

        if nums.len() != 3 {
            return Err(Error::ParseError);
        }

        let entry = MapEntry {
            source: nums[1]..(nums[1] + nums[2]),
            dest: nums[0]..(nums[0] + nums[2]),
        };
        map.entries.push(entry);
    }
    Ok(map)
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
    let mut lines = read_lines(file_path)?;
    let seeds = if let Some(seed_line) = lines.next() {
        let seed_line = seed_line?;

        if !seed_line.starts_with("seeds: ") {
            return Err(Error::ParseError)
        }

        seed_line.as_str()[("seeds: ".len())..].split_whitespace().map(str::parse::<u64>).collect::<Result<Vec<_>,_>>()?

    } else {
        return Err(Error::ParseError)
    };

    // Check to make sure there is a blank line next
    if let Some(line) = lines.next() {
        let line = line?;
        if !line.is_empty() {
            return Err(Error::ParseError)
        }
    } else {
        return Err(Error::ParseError)
    };


    Ok((seeds, Maps{
        seed_to_soil: load_map(&mut lines, "seed-to-soil map:")?,
        soil_to_fertilizer: load_map(&mut lines, "soil-to-fertilizer map:")?,
        fertilizer_to_water: load_map(&mut lines, "fertilizer-to-water map:")?,
        water_to_light: load_map(&mut lines, "water-to-light map:")?,
        light_to_temperature: load_map(&mut lines, "light-to-temperature map:")?,
        temperature_to_humidity: load_map(&mut lines, "temperature-to-humidity map:")?,
        humidity_to_location: load_map(&mut lines, "humidity-to-location map:")?,
    }))
}