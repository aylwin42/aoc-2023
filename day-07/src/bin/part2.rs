use clap::Parser;

use day07::args::Cli;
use day07::{Error, input, part2};

fn main() -> Result<(),Error> {
    let cli_args = Cli::parse();
    let input_data= input::load(cli_args.input_file)?;
    
    let part2_result = part2(&input_data)?;
    
    println!("Day 07 Part 2: {part2_result}");

    Ok(())
}