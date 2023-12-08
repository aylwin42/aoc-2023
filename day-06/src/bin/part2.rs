use clap::Parser;

use day06::args::Cli;
use day06::{Error, input, part2};

fn main() -> Result<(),Error> {
    let cli_args = Cli::parse();
    let input_data= input::load(cli_args.input_file)?;
    
    let part2_result = part2(&input_data)?;
    
    println!("Day 06 Part 2: {part2_result}");

    Ok(())
}