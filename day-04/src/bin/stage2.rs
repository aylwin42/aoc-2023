use clap::Parser;

use day04::args::Cli;
use day04::{Error, input, stage2};

fn main() -> Result<(),Error> {
    let cli_args = Cli::parse();
    let input_data= input::load(cli_args.input_file)?;
    
    let stage2_result = stage2(&input_data)?;
    
    println!("Day 04 Part 2: {stage2_result}");

    Ok(())
}