use clap::Parser;

use day04::args::Cli;
use day04::{Error, input, stage1};

fn main() -> Result<(),Error> {
    let cli_args = Cli::parse();
    let input_data= input::load(cli_args.input_file)?;
    
    let stage1_result = stage1(&input_data)?;
    
    println!("Day 04 Part 1: {stage1_result}");

    Ok(())
}