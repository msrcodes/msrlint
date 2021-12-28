use clap::Parser;

use crate::cli::Cli;

mod cli;

fn main() {
    let cli = Cli::parse();

    println!("Using input {:?}", cli.files);
}
