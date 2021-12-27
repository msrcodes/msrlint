use clap::StructOpt;

use crate::cli::Cli;

mod cli;

fn main() {
    let args = Cli::parse();

    println!("Hello {}!", args.no_color)
}
