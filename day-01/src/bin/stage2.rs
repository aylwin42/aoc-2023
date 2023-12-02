use clap::Parser;

use day01::args::Cli;
use day01::input;
use day01::Error;
use day01::stage2;

fn main() -> Result<(),Error> {
    let cli_args = Cli::parse();
    let input_data= input::load(cli_args.input_file)?;
    
    let stage2_result = stage2(&input_data)?;
    
    println!("Day 01 Stage 2: {stage2_result}");

    Ok(())
}