#![warn(clippy::pedantic)]
use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// File to read for the input data
    pub input_file: PathBuf
}