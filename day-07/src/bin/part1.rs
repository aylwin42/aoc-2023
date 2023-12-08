use clap::Parser;

use day07::args::Cli;
use day07::{Error, input, part1};

fn main() -> Result<(),Error> {
    let cli_args = Cli::parse();
    let input_data= input::load(cli_args.input_file)?;
    
    let part1_result = part1(&input_data)?;
    
    println!("Day 07 Part 1: {part1_result}");

    Ok(())
}